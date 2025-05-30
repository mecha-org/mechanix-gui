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
use libpulse_binding::volume::{ChannelVolumes, Volume};
use log::{error, info, trace, warn};
use std::cell::RefCell;
use std::rc::Rc;
use std::thread;


const APPLICATION_NAME: &str = "pulseaudio";
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
pub struct ServerInfo {
    /// User name of the daemon process.
    pub user_name: Option<String>,
    /// Host name the daemon is running on.
    pub host_name: Option<String>,
    /// Version string of the daemon.
    pub server_version: Option<String>,
    /// Server package name (usually “pulseaudio”).
    pub server_name: Option<String>,
    // Default sample specification.
    //pub sample_spec: sample::Spec,
    /// Name of default sink.
    pub default_sink_name: Option<String>,
    /// Name of default source.
    pub default_source_name: Option<String>,
    /// A random cookie for identifying this instance of PulseAudio.
    pub cookie: u32,
    // Default channel map.
    //pub channel_map: channelmap::Map,
}

impl<'a> From<&'a pulse::context::introspect::ServerInfo<'a>> for ServerInfo {
    fn from(info: &'a pulse::context::introspect::ServerInfo<'a>) -> Self {
        Self {
            user_name: info.user_name.as_ref().map(|cow| cow.to_string()),
            host_name: info.host_name.as_ref().map(|cow| cow.to_string()),
            server_version: info.server_version.as_ref().map(|cow| cow.to_string()),
            server_name: info.server_name.as_ref().map(|cow| cow.to_string()),
            //sample_spec: info.sample_spec,
            default_sink_name: info.default_sink_name.as_ref().map(|cow| cow.to_string()),
            default_source_name: info.default_source_name.as_ref().map(|cow| cow.to_string()),
            cookie: info.cookie,
            //channel_map: info.channel_map,
        }
    }
}


#[derive(Debug)]
pub enum Message {
    /// Get a list of output devices
    GetSinks,
    /// Response containing list of output devices or error
    SetSink(Result<Vec<DeviceInfo>, PulseAudioError>),

    /// Get a list of input devices
    GetSources,
    /// Response containing list of input devices or error
    SetSource(Result<Vec<DeviceInfo>, PulseAudioError>),

    /// Get the default output device
    GetDefaultSink,
    /// Response containing default output device or error
    SetDefaultSink(Result<DeviceInfo, PulseAudioError>),

    /// Get the default input device
    GetDefaultSource,
    /// Response containing default input device or error 
    SetDefaultSource(Result<DeviceInfo, PulseAudioError>),

    /// Set volume for a specific output device by name
    SetSinkVolumeByName(String, ChannelVolumes),
    /// Set volume for a specific input device by name
    SetSourceVolumeByName(String, ChannelVolumes),

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
    // Create pulse server thread, and bidirectional comms
    pub fn new() -> Self {
        let (to_pulse, mut to_pulse_recv) = tokio::sync::mpsc::channel(50);
        let (from_pulse_send, from_pulse) = tokio::sync::mpsc::channel(50);

        // this thread should complete by pushing a completed message,
        // or fail message. This should never complete/fail without pushing
        // a message. This lets the iced subscription go to sleep while init
        // finishes. TLDR: be very careful with error handling
        thread::spawn(move || {
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap();

            // take `PulseServer` and handle reciver into async context
            // to listen for messages that need to be passed to the pulseserver
            // this lets us put the thread to sleep, but keep hold a single
            // thread, because pulse audio's API is not multithreaded... at all
            rt.block_on(async {
                let mut server = match PulseServer::connect().and_then(|s| s.init()) {
                    Ok(server) => {
                        info!("connected to pulse server");
                        server
                    }
                    Err(err) => {
                        error!("failed to connect/init server: {:?}", err);
                        return;
                    }
                };
                let mut msgs = Vec::new();

                loop {
                    if let Some(msg) = to_pulse_recv.recv().await {
                        info!("received message: {:?}", msg);
                        msgs.push(msg);
                    }

                    for msg in msgs.drain(..) {
                        match msg {
                            Message::GetSinks => {
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
                            Message::GetSources => {
                                match server.get_sources() {
                                    Ok(sources) => {
                                        trace!("source count: {}", sources.len());
                                        // Send the sinks to the receiver
                                        if let Err(err) =
                                            from_pulse_send.send(Message::SetSource(Ok(sources))).await
                                        {
                                            error!("failed to send sinks: {:?}", err);
                                        };
                                        info!("sent sinks to receiver");
                                    }
                                    Err(e) => {
                                        error!("failed to get sinks: {:?}", e);
                                        // Send the error to the receiver
                                        if let Err(err) = from_pulse_send
                                            .send(Message::SetSource({
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
                            Message::GetDefaultSink => {
                                match server.get_default_sink() {
                                    Ok(sink) => {
                                        trace!("sink: {:?}", sink);
                                        // Send the sinks to the receiver
                                        if let Err(err) =
                                            from_pulse_send.send(Message::SetDefaultSink(Ok(sink))).await
                                        {
                                            error!("failed to send sinks: {:?}", err);
                                        };
                                        info!("sent sinks to receiver");
                                    }
                                    Err(e) => {
                                        error!("failed to get sinks: {:?}", e);
                                        // Send the error to the receiver
                                        if let Err(err) = from_pulse_send
                                            .send(Message::SetDefaultSink({
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
                            Message::GetDefaultSource => {
                                match server.get_default_source() {
                                    Ok(sink) => {
                                        trace!("sink: {:?}", sink);
                                        // Send the sinks to the receiver
                                        if let Err(err) =
                                            from_pulse_send.send(Message::SetDefaultSource(Ok(sink))).await
                                        {
                                            error!("failed to send sinks: {:?}", err);
                                        };
                                        info!("sent sinks to receiver");
                                    }
                                    Err(e) => {
                                        error!("failed to get sinks: {:?}", e);
                                        // Send the error to the receiver
                                        if let Err(err) = from_pulse_send
                                            .send(Message::SetDefaultSource({
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
                            Message::SetSinkVolumeByName(name, channel_volumes) => {
                                server.set_sink_volume_by_name(&name, &channel_volumes)
                            }
                            Message::SetSourceVolumeByName(name, channel_volumes) => {
                                server.set_source_volume_by_name(&name, &channel_volumes)
                            }
                            _ => {
                                warn!("message doesn't match")
                            }
                        }
                    }
                }
            });
        });
        Self {
            to_pulse,
            from_pulse,
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

    #[error("failed to create proplist")]
    FailedToCreateProplist,
}

// `PulseServer` code is heavily inspired by Dave Patrick Caberto's pulsectl-rs (SeaDve)
// https://crates.io/crates/pulsectl-rs
impl PulseServer {
    // connect() requires init() to be run after
    pub fn connect() -> Result<Self, PulseServerError> {
        info!("connecting to pulse server");
        let mut proplist = match Proplist::new() {
            Some(p) => p,
            None => return Err(PulseServerError::FailedToCreateProplist),
        };
        proplist
            .set_str(
                pulse::proplist::properties::APPLICATION_NAME,
                APPLICATION_NAME,
            )
            .or(Err(PulseServerError::Connect))?;

        // Create a mainloop and context for PulseAudio
        let mainloop = Rc::new(RefCell::new(
            pulse::mainloop::standard::Mainloop::new().ok_or(PulseServerError::Connect)?,
        ));

        // Create a context with the mainloop and proplist
        let context = Rc::new(RefCell::new(
            Context::new_with_proplist(&*mainloop.borrow(), "MainConn", &proplist)
                .ok_or(PulseServerError::Connect)?,
        ));

        // Create an introspector for the context
        let introspector = context.borrow_mut().introspect();

        // Connect to the PulseAudio server
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

    fn get_default_sink(&mut self) -> Result<DeviceInfo, PulseServerError> {
        let server_info = self.get_server_info();
        match server_info {
            Ok(info) => {
                let name = &info.default_sink_name.unwrap_or_default();
                let device = Rc::new(RefCell::new(Some(None)));
                let dev_ref = device.clone();
                let op = self.introspector.get_sink_info_by_name(
                    name,
                    move |sink_list: ListResult<&SinkInfo>| {
                        if let ListResult::Item(item) = sink_list {
                            dev_ref.borrow_mut().as_mut().unwrap().replace(item.into());
                        }
                    },
                );
                self.wait_for_result(op)?;
                let mut result = device.borrow_mut();
                result.take().unwrap().ok_or({
                    PulseServerError::Misc("get_default_sink(): Error getting requested device".to_string())
                })
            }
            Err(_) => Err(PulseServerError::Misc("get_default_sink() failed".to_string())),
        }
    }

    fn get_default_source(&mut self) -> Result<DeviceInfo, PulseServerError> {
        let server_info = self.get_server_info();
        match server_info {
            Ok(info) => {
                let name = &info.default_source_name.unwrap_or_default();
                let device = Rc::new(RefCell::new(Some(None)));
                let dev_ref = device.clone();
                let op = self.introspector.get_source_info_by_name(
                    name,
                    move |source_list: ListResult<&SourceInfo>| {
                        if let ListResult::Item(item) = source_list {
                            dev_ref.borrow_mut().as_mut().unwrap().replace(item.into());
                        }
                    },
                );
                self.wait_for_result(op)?;
                let mut result = device.borrow_mut();
                result.take().unwrap().ok_or({
                    PulseServerError::Misc("get_default_source(): Error getting requested device".to_string())
                })
            }
            Err(_) => Err(PulseServerError::Misc("get_default_source() failed".to_string())),
        }
    }

    pub fn get_server_info(&mut self) -> Result<ServerInfo, PulseServerError> {
        let info = Rc::new(RefCell::new(Some(None)));
        let info_ref = info.clone();

        let op = self.introspector.get_server_info(move |res| {
            info_ref.borrow_mut().as_mut().unwrap().replace(res.into());
        });
        self.wait_for_result(op)?;
        info.take()
            .flatten()
            .ok_or(PulseServerError::Misc("get_server_info(): failed".to_string()))
    }


    fn set_sink_volume_by_name(&mut self, name: &str, volume: &ChannelVolumes) {
        let op = self
            .introspector
            .set_sink_mute_by_name(name, volume.is_muted(), None);
        self.wait_for_result(op).ok();

        let op = self
            .introspector
            .set_sink_volume_by_name(name, volume, None);
        self.wait_for_result(op).ok();
    }


    /// Sets the volume and mute state for a PulseAudio source (input device) identified by its name.
    ///
    /// This method performs two operations:
    /// 1. Sets the mute state of the source based on the volume's mute status
    /// 2. Sets the volume levels for all channels of the source
    ///
    /// # Parameters
    /// * `name` - The name of the PulseAudio source to modify
    /// * `volume` - The new volume levels and mute state to apply
    ///
    /// # Note
    /// Both operations are performed independently and their results are ignored.
    /// If either operation fails, no error will be propagated.
    fn set_source_volume_by_name(&mut self, name: &str, volume: &ChannelVolumes) {
        let op = self
            .introspector
            .set_source_mute_by_name(name, volume.is_muted(), None);
        let _ = self.wait_for_result(op);

        let op = self
            .introspector
            .set_source_volume_by_name(name, volume, None);
        let _ = self.wait_for_result(op);
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
