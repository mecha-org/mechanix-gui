extern crate libpulse_binding as pulse;
use crate::errors::PulseAudioError;
use anyhow::Result;
use libpulse_binding::callbacks::ListResult;
use libpulse_binding::context::introspect::{Introspector, SinkInfo, SourceInfo};
use libpulse_binding::context::Context;
use libpulse_binding::error::PAErr;
use libpulse_binding::mainloop::standard::{IterateResult, Mainloop};
use libpulse_binding::proplist::Proplist;
use libpulse_binding::volume::Volume;
use log::{error, info};
use std::cell::RefCell;
use std::rc::Rc;
use std::thread;
use tokio::sync::{mpsc, oneshot};

const APPLICATION_NAME: &str = "pulseaudio";
#[derive(Debug, Clone, PartialEq, Default)]
pub struct DeviceInfo {
    pub name: Option<String>,
    pub description: Option<String>,
    pub volume: f64,
    pub mute: bool,
    pub index: u32,
}

impl<'a> From<&SinkInfo<'a>> for DeviceInfo {
    fn from(info: &SinkInfo<'a>) -> Self {
        Self {
            name: info.name.clone().map(|x| x.into_owned()),
            description: info.description.clone().map(|x| x.into_owned()),
            volume: volume_to_percentage(info.volume.max()),
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
            volume: volume_to_percentage(info.volume.max()),
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
    GetSinks {
        reply: oneshot::Sender<Result<Vec<DeviceInfo>, PulseServerError>>,
    },

    /// Get a list of input devices
    GetSources {
        reply: oneshot::Sender<Result<Vec<DeviceInfo>, PulseServerError>>,
    },

    /// Get the default output device
    GetDefaultSink {
        reply: oneshot::Sender<Result<DeviceInfo, PulseServerError>>,
    },

    /// Get the default input device
    GetDefaultSource {
        reply: oneshot::Sender<Result<DeviceInfo, PulseServerError>>,
    },

    /// Response containing default output device or error
    SetDefaultSink {
        name: String,
        reply: oneshot::Sender<Result<bool, PulseAudioError>>,
    },

    /// Response containing default input device or error
    SetDefaultSource {
        name: String,
        reply: oneshot::Sender<Result<bool, PulseAudioError>>,
    },

    // Volume control
    SetSinkVolumeByName {
        name: String,
        volume: f32,
        reply: oneshot::Sender<Result<(), PulseServerError>>,
    },
    SetSourceVolumeByName {
        name: String,
        volume: f32,
        reply: oneshot::Sender<Result<(), PulseServerError>>,
    },
    MuteSinkByName {
        name: String,
        reply: oneshot::Sender<Result<(), PulseServerError>>,
    },
    MuteSourceByName {
        name: String,
        reply: oneshot::Sender<Result<(), PulseServerError>>,
    },
    UnMuteSinkByName {
        name: String,
        reply: oneshot::Sender<Result<(), PulseServerError>>,
    },
    UnMuteSourceByName {
        name: String,
        reply: oneshot::Sender<Result<(), PulseServerError>>,
    },

    Shutdown,
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

pub struct PulseHandle {
    tx: mpsc::Sender<Message>,
}

impl PulseHandle {
    pub async fn get_sinks(&self) -> Result<Vec<DeviceInfo>, PulseServerError> {
        let (reply, rx) = oneshot::channel();
        self.tx
            .send(Message::GetSinks { reply })
            .await
            .map_err(|_| PulseServerError::Misc("worker gone".into()))?;
        rx.await
            .map_err(|_| PulseServerError::Misc("worker dropped reply".into()))?
    }

    pub async fn get_sources(&self) -> Result<Vec<DeviceInfo>, PulseServerError> {
        let (reply, rx) = oneshot::channel();
        self.tx
            .send(Message::GetSources { reply })
            .await
            .map_err(|_| PulseServerError::Misc("worker gone".into()))?;
        rx.await
            .map_err(|_| PulseServerError::Misc("worker dropped reply".into()))?
    }

    pub async fn get_default_sink(&self) -> Result<DeviceInfo, PulseServerError> {
        let (reply, rx) = oneshot::channel();
        self.tx
            .send(Message::GetDefaultSink { reply })
            .await
            .map_err(|_| PulseServerError::Misc("worker gone".into()))?;
        rx.await
            .map_err(|_| PulseServerError::Misc("worker dropped reply".into()))?
    }
    pub async fn get_default_source(&self) -> Result<DeviceInfo, PulseServerError> {
        let (reply, rx) = oneshot::channel();
        self.tx
            .send(Message::GetDefaultSource { reply })
            .await
            .map_err(|_| PulseServerError::Misc("worker gone".into()))?;
        rx.await
            .map_err(|_| PulseServerError::Misc("worker dropped reply".into()))?
    }
    pub async fn set_sink_volume_by_name(
        &self,
        name: &str,
        volume: &f32,
    ) -> Result<(), PulseServerError> {
        let (reply, rx) = oneshot::channel();
        self.tx
            .send(Message::SetSinkVolumeByName {
                name: name.to_string(),
                volume: *volume,
                reply,
            })
            .await
            .map_err(|_| PulseServerError::Misc("worker gone".into()))?;
        rx.await
            .map_err(|_| PulseServerError::Misc("worker dropped reply".into()))?
    }
    pub async fn set_source_volume_by_name(
        &self,
        name: &str,
        volume: &f32,
    ) -> Result<(), PulseServerError> {
        let (reply, rx) = oneshot::channel();
        self.tx
            .send(Message::SetSourceVolumeByName {
                name: name.to_string(),
                volume: *volume,
                reply,
            })
            .await
            .map_err(|_| PulseServerError::Misc("worker gone".into()))?;
        rx.await
            .map_err(|_| PulseServerError::Misc("worker dropped reply".into()))?
    }

    pub async fn set_default_source_by_name(&self, name: &str) -> Result<bool, PulseAudioError> {
        let (reply, rx) = oneshot::channel();
        self.tx
            .send(Message::SetDefaultSource {
                name: name.to_string(),
                reply,
            })
            .await
            .map_err(|_| PulseServerError::Misc("worker gone".into()))?;
        rx.await
            .map_err(|_| PulseServerError::Misc("worker dropped reply".into()))?
    }
    pub async fn set_default_sink_by_name(&self, name: &str) -> Result<bool, PulseAudioError> {
        let (reply, rx) = oneshot::channel();
        self.tx
            .send(Message::SetDefaultSink {
                name: name.to_string(),
                reply,
            })
            .await
            .map_err(|_| PulseServerError::Misc("worker gone".into()))?;
        rx.await
            .map_err(|_| PulseServerError::Misc("worker dropped reply".into()))?
    }
    pub async fn set_sink_mute_by_name(&self, name: &str) -> Result<(), PulseServerError> {
        let (reply, rx) = oneshot::channel();
        self.tx
            .send(Message::MuteSinkByName {
                name: name.to_string(),
                reply,
            })
            .await
            .map_err(|_| PulseServerError::Misc("worker gone".into()))?;
        rx.await
            .map_err(|_| PulseServerError::Misc("worker dropped reply".into()))?
    }

    pub async fn set_sink_unmute_by_name(&self, name: &str) -> Result<(), PulseServerError> {
        let (reply, rx) = oneshot::channel();
        self.tx
            .send(Message::UnMuteSinkByName {
                name: name.to_string(),
                reply,
            })
            .await
            .map_err(|_| PulseServerError::Misc("worker gone".into()))?;
        rx.await
            .map_err(|_| PulseServerError::Misc("worker dropped reply".into()))?
    }
    pub async fn mute_source_by_name(&self, name: &str) -> Result<(), PulseServerError> {
        let (reply, rx) = oneshot::channel();
        self.tx
            .send(Message::MuteSourceByName {
                name: name.to_string(),
                reply,
            })
            .await
            .map_err(|_| PulseServerError::Misc("worker gone".into()))?;
        rx.await
            .map_err(|_| PulseServerError::Misc("worker dropped reply".into()))?
    }

    pub async fn set_source_unmute_by_name(&self, name: &str) -> Result<(), PulseServerError> {
        let (reply, rx) = oneshot::channel();
        self.tx
            .send(Message::UnMuteSourceByName {
                name: name.to_string(),
                reply,
            })
            .await
            .map_err(|_| PulseServerError::Misc("worker gone".into()))?;
        rx.await
            .map_err(|_| PulseServerError::Misc("worker dropped reply".into()))?
    }
    pub async fn shutdown(&self) {
        // Best-effort shutdown signal; ignore error if worker already gone
        let _ = self.tx.send(Message::Shutdown).await;
    }
}

pub fn spawn_pulse_worker() -> Result<PulseHandle, PulseInitError> {
    let (tx, mut rx) = mpsc::channel::<Message>(64);

    thread::spawn(move || {
        let server = match PulseServer::connect().and_then(|s| s.init()) {
            Ok(s) => s,
            Err(e) => {
                log::error!("pulse connect/init failed: {e:?}");
                return;
            }
        };
        let mut server = server;
        while let Some(cmd) = rx.blocking_recv() {
            match cmd {
                Message::GetSinks { reply } => {
                    let _ = reply.send(server.get_sinks());
                }
                Message::GetSources { reply } => {
                    let _ = reply.send(server.get_sources());
                }
                Message::GetDefaultSink { reply } => {
                    let _ = reply.send(server.get_default_sink());
                }
                Message::GetDefaultSource { reply } => {
                    let _ = reply.send(server.get_default_source());
                }
                Message::SetSinkVolumeByName {
                    name,
                    volume,
                    reply,
                } => {
                    server.set_sink_volume_by_name(&name, &volume);
                    let _ = reply.send(Ok(()));
                }
                Message::SetSourceVolumeByName {
                    name,
                    volume,
                    reply,
                } => {
                    server.set_source_volume_by_name(&name, &volume);
                    let _ = reply.send(Ok(()));
                }
                Message::MuteSinkByName { name, reply } => {
                    server.mute_sink_by_name(&name);
                    let _ = reply.send(Ok(()));
                }
                Message::MuteSourceByName { name, reply } => {
                    server.mute_source_by_name(&name);
                    let _ = reply.send(Ok(()));
                }
                Message::Shutdown => {
                    if let Err(e) = server.shutdown() {
                        log::warn!("PulseServer shutdown error: {e:?}");
                    }
                    break;
                }
                Message::SetDefaultSink { name, reply } => {
                    let _ = server.set_default_sink(&name);
                    let _ = reply.send(Ok(true));
                }
                Message::SetDefaultSource { name, reply} => {
                    let _ = server.set_default_source(&name);
                    let _ = reply.send(Ok(true));
                }
                Message::UnMuteSinkByName { name, reply } => {
                    server.unmute_sink_by_name(&name);
                    let _ = reply.send(Ok(()));
                }
                Message::UnMuteSourceByName { name, reply } => {
                    server.unmute_source_by_name(&name);
                    let _ = reply.send(Ok(()));
                }
            }
        }
    });

    Ok(PulseHandle { tx })
}
pub struct PulseAudioService {
    pub handle: PulseHandle,
}
impl PulseAudioService {
    pub fn new() -> Result<Self, PulseInitError> {
        let handle = spawn_pulse_worker()?;
        Ok(Self { handle })
    }
}

pub struct PulseServer {
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

    pub fn get_default_sink(&mut self) -> Result<DeviceInfo, PulseServerError> {
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
                    PulseServerError::Misc(
                        "get_default_sink(): Error getting requested device".to_string(),
                    )
                })
            }
            Err(_) => Err(PulseServerError::Misc(
                "get_default_sink() failed".to_string(),
            )),
        }
    }

    pub fn get_default_source(&mut self) -> Result<DeviceInfo, PulseServerError> {
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
                    PulseServerError::Misc(
                        "get_default_source(): Error getting requested device".to_string(),
                    )
                })
            }
            Err(_) => Err(PulseServerError::Misc(
                "get_default_source() failed".to_string(),
            )),
        }
    }

    /// Gets information about the PulseAudio server.
    ///
    /// # Returns
    ///
    /// A `ServerInfo` containing information about the server.
    ///
    /// # Errors
    ///
    /// `PulseServerError::Misc` if an error occurs while retrieving the server
    /// information.
    pub fn get_server_info(&mut self) -> Result<ServerInfo, PulseServerError> {
        let info = Rc::new(RefCell::new(Some(None)));
        let info_ref = info.clone();

        let op = self.introspector.get_server_info(move |res| {
            info_ref.borrow_mut().as_mut().unwrap().replace(res.into());
        });
        self.wait_for_result(op)?;
        info.take().flatten().ok_or(PulseServerError::Misc(
            "get_server_info(): failed".to_string(),
        ))
    }

    pub fn set_sink_volume_by_name(&mut self, name: &str, volume_to_set: &f32) {
        if *volume_to_set <= 0.0 {
            // Mute the SINK (was calling source mute by mistake)
            let op = self.introspector.set_sink_mute_by_name(name, true, None);
            let _ = self.wait_for_result(op);
            return;
        }
        let target = Rc::new(RefCell::new(None::<(u32, pulse::volume::ChannelVolumes)>));
        let target_ref = target.clone();

        let volume_value = *volume_to_set; //
        println!("volume_value: {}", volume_value);
        // Fetch current sink info and prepare the new per-channel volumes
        let op = self.introspector.get_sink_info_by_name(name, move |res| {
            if let ListResult::Item(device) = res {
                let mut current_volume = device.volume;
                let mut avg = current_volume.avg();
                avg.0 = ((volume_value * 0.01) * 65536.0) as u32;
                for i in 1..=current_volume.len() {
                    current_volume.set(i, avg);
                }
                target_ref
                    .borrow_mut()
                    .replace((device.index, current_volume));
            }
        });
        let _ = self.wait_for_result(op);
        // Apply the volume and unmute the sink
        let target = target.borrow_mut().take();
        if let Some((idx, v)) = target {
            let op_set = self.introspector.set_sink_volume_by_index(idx, &v, None);
            let _ = self.wait_for_result(op_set);
            // Ensure it’s unmuted when setting a positive volume
            let op_unmute = self.introspector.set_sink_mute_by_index(idx, false, None);
            let _ = self.wait_for_result(op_unmute);
        }
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
    pub fn set_source_volume_by_name(&mut self, name: &str, volume_to_set: &f32) {
        info!("set_source_volume_by_name: {} {}", name, volume_to_set);
        if *volume_to_set <= 0.0 {
            // Mute the SINK (was calling source mute by mistake)
            let op = self.introspector.set_source_mute_by_name(name, true, None);
            let _ = self.wait_for_result(op);
            return;
        }
        let target = Rc::new(RefCell::new(None::<(u32, pulse::volume::ChannelVolumes)>));
        let target_ref = target.clone();
        let volume_value = *volume_to_set; //
        let op = self.introspector.get_source_info_by_name(name, move |res| {
            if let ListResult::Item(device) = res {
                let mut current_volume = device.volume;
                let mut avg = current_volume.avg();
                avg.0 = ((volume_value * 0.01) * 65536.0) as u32;
                for i in 1..=current_volume.len() {
                    current_volume.set(i, avg);
                }
                target_ref
                    .borrow_mut()
                    .replace((device.index, current_volume));
            }
        });
        let _ = self.wait_for_result(op);
        // Apply the volume and unmute the sink
        let target = target.borrow_mut().take();
        if let Some((idx, v)) = target {
            let op_set = self.introspector.set_source_volume_by_index(idx, &v, None);
            let _ = self.wait_for_result(op_set);
            // Ensure it’s unmuted when setting a positive volume
            let op_unmute = self.introspector.set_source_mute_by_index(idx, false, None);
            let _ = self.wait_for_result(op_unmute);
        }
        info!("set_source_volume_by_name: done");
    }
    pub fn mute_sink_by_name(&mut self, name: &str) {
        let op = self.introspector.set_sink_mute_by_name(name, true, None);
        let _ = self.wait_for_result(op);
    }
    pub fn unmute_sink_by_name(&mut self, name: &str) {
        let op = self.introspector.set_sink_mute_by_name(name, false, None);
        let _ = self.wait_for_result(op);
    }
    pub fn mute_source_by_name(&mut self, name: &str) {
        info!("mute_source_by_name: {}", name);
        let op = self.introspector.set_source_mute_by_name(name, true, None);
        let _ = self.wait_for_result(op);
        info!("mute_source_by_name: done");
    }
    pub fn unmute_source_by_name(&mut self, name: &str) {
        let op = self.introspector.set_source_mute_by_name(name, false, None);
        let _ = self.wait_for_result(op);
    }

    pub fn set_default_sink(&mut self, name: &str) {
        let result = self.context.borrow_mut().set_default_sink(name, move |x| {
            info!("set_default_sink: {:?}", x);
        });
        let _ = self.wait_for_result(result);
    }
    pub fn set_default_source(&mut self, source_name: &str) {
        let result = self
            .context
            .borrow_mut()
            .set_default_source(source_name, move |x| {
                info!("set_default_source: {:?}", x);
            });
         let _ = self.wait_for_result(result);
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

    pub fn shutdown(&mut self) -> Result<(), PulseServerError> {
        // Ask PulseAudio to disconnect
        self.context.borrow_mut().disconnect();
        // Drive the mainloop until the context reports Terminated
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
                pulse::context::State::Terminated => break,
                pulse::context::State::Failed => {
                    return Err(PulseServerError::ContextErr(pulse::context::State::Failed))
                }
                _ => {}
            }
        }
        Ok(())
    }
}

/// Convert a [`Volume`] to a percentage as `f64`.
pub fn volume_to_percentage(volume: Volume) -> f64 {
    let range = Volume::NORMAL.0 as f64 - Volume::MUTED.0 as f64;
    (volume.0 as f64 - Volume::MUTED.0 as f64) * 100.0 / range
}

/// Convert a percentage to a [`Volume`].
pub fn percentage_to_volume(factor: f64) -> Volume {
    let range = Volume::NORMAL.0 as f64 - Volume::MUTED.0 as f64;
    Volume((Volume::MUTED.0 as f64 + factor * range / 100.0) as u32)
}
