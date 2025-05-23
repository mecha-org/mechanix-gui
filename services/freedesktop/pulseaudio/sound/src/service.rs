extern crate libpulse_binding as pulse;
use crate::errors::PulseAudioError;
use crate::service::Message::SetSink;
use anyhow::Result;
use libpulse_binding::callbacks::ListResult;
use libpulse_binding::context::introspect::{Introspector, SinkInfo, SourceInfo};
use libpulse_binding::context::Context;
use libpulse_binding::error::PAErr;
use libpulse_binding::mainloop::standard::{IterateResult, Mainloop};
use libpulse_binding::proplist::Proplist;
use libpulse_binding::volume::ChannelVolumes;
use log::{error, info, trace, warn};
use std::cell::RefCell;
use std::rc::Rc;
use std::thread;
use tokio::sync::mpsc;

#[derive(Debug, Clone, PartialEq)]
pub struct DeviceInfo {
    pub name: Option<String>,
    pub description: Option<String>,
    pub volume: ChannelVolumes,
    pub mute: bool,
    pub index: u32,
}

impl<'a> From<&SinkInfo<'a>> for DeviceInfo {
    fn from(info: &SinkInfo<'a>) -> Self {
        Self {
            name: info.name.clone().map(|x| x.into_owned()),
            description: info.description.clone().map(|x| x.into_owned()),
            volume: info.volume,
            mute: info.mute,
            index: info.index,
        }
    }
}

impl<'a> From<&SourceInfo<'a>> for DeviceInfo {
    fn from(info: &SourceInfo<'a>) -> Self {
        Self {
            name: info.name.clone().map(|x| x.into_owned()),
            description: info.description.clone().map(|x| x.into_owned()),
            volume: info.volume,
            mute: info.mute,
            index: info.index,
        }
    }
}

impl Eq for DeviceInfo {}

#[derive(Debug)]
pub enum Message {
    /// Get a list of output devices
    GetSinks,
    SetSink(Result<Vec<DeviceInfo>, PulseAudioError>),
}
pub struct PulseHandle {
    pub to_pulse: tokio::sync::mpsc::Sender<Message>,
    pub from_pulse: tokio::sync::mpsc::Receiver<Message>,
}
#[derive(Debug, thiserror::Error)]
pub enum PulseInitError {
    /// Failed to spawn the thread
    #[error("failed to spawn thread")]
    ThreadSpawnFailed,

    /// Failed to initialize the PulseAudio server
    #[error("failed to initialize PulseAudio server: {0}")]
    InitFailed(String),
}

impl PulseHandle {
    /// Create a pulse server thread and bidirectional comms.
    /// Returns an error if initialization fails.
    pub fn new() -> Result<Self, PulseInitError> {
        // Use a oneshot channel to signal init result
        let (init_tx, mut init_rx) = mpsc::channel(10);
        let (to_pulse, mut to_pulse_recv) = tokio::sync::mpsc::channel(50);
        let (from_pulse_send, from_pulse) = tokio::sync::mpsc::channel(50);

        let thread_result = thread::Builder::new()
            .name("pulse-server".to_string())
            .spawn(move || {
                let rt = tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                    .expect("Failed to build runtime");

                // All PulseAudio objects must remain on this thread
                rt.block_on(async {
                    match PulseServer::connect().and_then(|s| s.init()) {
                        Ok(server) => {
                            info!("connected to pulse server");
                            // Signal successful initialization
                            let _ = init_tx.send(Ok(()));
                            // Main message loop
                            while let Some(request) = to_pulse_recv.recv().await {
                                match request {
                                    Message::GetSinks => {
                                        trace!("request to get sinks");
                                        match server.get_sinks() {
                                            Ok(sinks) => {
                                                trace!("sink count: {}", sinks.len());
                                                // Send the sinks to the receiver
                                                if let Err(err) =
                                                    from_pulse_send.send(SetSink(Ok(sinks))).await
                                                {
                                                    error!("failed to send sinks: {:?}", err);
                                                };
                                                info!("sent sinks to receiver");
                                            }
                                            Err(e) => {
                                                error!("failed to get sinks: {:?}", e);
                                                // Send the error to the receiver
                                                if let Err(err) = from_pulse_send
                                                    .send(Message::SetSink({
                                                        Err(PulseAudioError::SendRequestError(
                                                            format!("{:?}", e),
                                                        ))
                                                    }))
                                                    .await
                                                {
                                                    error!("failed to send error: {:?}", err);
                                                };
                                            }
                                        }
                                    }
                                    _ => {
                                        warn!("message doesn't match")
                                    }
                                }
                            }
                        }
                        Err(err) => {
                            error!("failed to connect/init server: {:?}", err);
                            // Signal initialization failure
                            let _ =
                                init_tx.send(Err(PulseInitError::InitFailed(format!("{:?}", err))));
                        }
                    }
                });
            });

        if thread_result.is_err() {
            return Err(PulseInitError::ThreadSpawnFailed);
        }

        // Wait for a thread to signal an initialization result
        match init_rx.try_recv() {
            Ok(Ok(())) => Ok(Self {
                to_pulse,
                from_pulse,
            }),
            Ok(Err(e)) => Err(e),
            Err(_) => Err(PulseInitError::ThreadSpawnFailed),
        }
    }
}
struct PulseServer {
    mainloop: Rc<RefCell<Mainloop>>,
    context: Rc<RefCell<Context>>,
    introspector: Introspector,
}

#[derive(Clone, thiserror::Error, Debug)]
pub enum PulseServerError {
    #[error("iteration error: {0:?}")]
    IterateErr(IterateResult),

    #[error("context error: {0:?}")]
    ContextErr(pulse::context::State),

    #[error("operation error: {0:?}")]
    OperationErr(pulse::operation::State),

    #[error("PA error: {0}")]
    PAErr(PAErr),

    #[error("connection error")]
    Connect,

    #[error("misc error: {0}")]
    Misc(String),
}

// `PulseServer` code is heavily inspired by Dave Patrick Caberto's pulsectl-rs (SeaDve)
// https://crates.io/crates/pulsectl-rs
impl PulseServer {
    // connect() requires init() to be run after
    pub fn connect() -> Result<Self, PulseServerError> {
        // TODO: fix app name, should be variable
        let mut proplist = Proplist::new().unwrap();
        proplist
            .set_str(
                pulse::proplist::properties::APPLICATION_NAME,
                "com.system76",
            )
            .or(Err(PulseServerError::Connect))?;

        let mainloop = Rc::new(RefCell::new(
            pulse::mainloop::standard::Mainloop::new().ok_or(PulseServerError::Connect)?,
        ));

        let context = Rc::new(RefCell::new(
            Context::new_with_proplist(&*mainloop.borrow(), "MainConn", &proplist)
                .ok_or(PulseServerError::Connect)?,
        ));

        let introspector = context.borrow_mut().introspect();

        context
            .borrow_mut()
            .connect(None, pulse::context::FlagSet::NOFLAGS, None)
            .map_err(PulseServerError::PAErr)?;

        Ok(Self {
            mainloop,
            context,
            introspector,
        })
    }

    /// Wait for pulse audio connection to complete
    pub fn init(self) -> Result<Self, PulseServerError> {
        loop {
            match self.mainloop.borrow_mut().iterate(false) {
                IterateResult::Success(_) => {}
                IterateResult::Err(e) => {
                    return Err(PulseServerError::IterateErr(IterateResult::Err(e)))
                }
                IterateResult::Quit(e) => {
                    return Err(PulseServerError::IterateErr(IterateResult::Quit(e)))
                }
            }

            match self.context.borrow().get_state() {
                pulse::context::State::Ready => break,
                pulse::context::State::Failed => {
                    return Err(PulseServerError::ContextErr(pulse::context::State::Failed))
                }
                pulse::context::State::Terminated => {
                    return Err(PulseServerError::ContextErr(
                        pulse::context::State::Terminated,
                    ))
                }
                _ => {}
            }
        }
        Ok(self)
    }

    /// Get a list of output devices
    pub fn get_sinks(&self) -> Result<Vec<DeviceInfo>, PulseServerError> {
        info!("get sinks init");
        let list: Rc<RefCell<Option<Vec<DeviceInfo>>>> = Rc::new(RefCell::new(Some(Vec::new())));
        let list_ref = list.clone();

        let operation = self.introspector.get_sink_info_list(
            move |sink_list: ListResult<&pulse::context::introspect::SinkInfo>| {
                if let ListResult::Item(item) = sink_list {
                    list_ref.borrow_mut().as_mut().unwrap().push(item.into());
                }
            },
        );
        self.wait_for_result(operation).and_then(|_| {
            list.borrow_mut().take().ok_or(PulseServerError::Misc(
                "get_sinks(): failed to wait for operation".to_string(),
            ))
        })
    }

    /// Get a list of input devices
    pub fn get_sources(&self) -> Result<Vec<DeviceInfo>, PulseServerError> {
        let list: Rc<RefCell<Option<Vec<DeviceInfo>>>> = Rc::new(RefCell::new(Some(Vec::new())));
        let list_ref = list.clone();

        let operation = self.introspector.get_source_info_list(
            move |sink_list: ListResult<&pulse::context::introspect::SourceInfo>| {
                if let ListResult::Item(item) = sink_list {
                    list_ref.borrow_mut().as_mut().unwrap().push(item.into());
                }
            },
        );
        self.wait_for_result(operation).and_then(|_| {
            list.borrow_mut().take().ok_or(PulseServerError::Misc(
                "get_sources(): Failed to wait for operation".to_string(),
            ))
        })
    }

    // after building an operation such as get_devices() we need to keep polling
    // the pulse audio server to "wait" for the operation to complete
    fn wait_for_result<G: ?Sized>(
        &self,
        operation: pulse::operation::Operation<G>,
    ) -> Result<(), PulseServerError> {
        // TODO: make this loop async. It is already in an async context, so
        // we could make this thread sleep while waiting for the pulse server's
        // response.
        loop {
            match self.mainloop.borrow_mut().iterate(false) {
                IterateResult::Err(e) => {
                    return Err(PulseServerError::IterateErr(IterateResult::Err(e)))
                }
                IterateResult::Quit(e) => {
                    return Err(PulseServerError::IterateErr(IterateResult::Quit(e)))
                }
                IterateResult::Success(_) => {}
            }
            match operation.get_state() {
                pulse::operation::State::Done => return Ok(()),
                pulse::operation::State::Running => {}
                pulse::operation::State::Cancelled => {
                    return Err(PulseServerError::OperationErr(
                        pulse::operation::State::Cancelled,
                    ))
                }
            }
        }
    }
}
