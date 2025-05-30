use crate::errors::TimeDateError;
use crate::proxies::timedate1Proxy;
use crate::service::TimeDateService;
use anyhow::{bail, Result};
use log::{error, info};
use tokio::sync::oneshot;
use tokio::{select, sync::mpsc};
use zbus::Connection;

/// Represents different types of timedate requests that can be handled by `TimeDateClient`.
#[derive(Debug)]
#[non_exhaustive]
pub enum TimeDateRequest {
    ListTimezones {
        /// Channel to send the result of the operation.
        reply_to: oneshot::Sender<Result<Vec<String>, TimeDateError>>,
    },
    SetTimezone {
        /// The timezone to set.
        timezone: String,
        /// Whether the operation is interactive.
        interactive: bool,
        /// Channel to send the result of the operation.
        reply_to: oneshot::Sender<Result<(), TimeDateError>>,
    },
    GetTimeZone {
        /// Channel to send the result of the operation.
        reply_to: oneshot::Sender<Result<String, TimeDateError>>,
    },
    SetLocalRTC {
        /// Whether to set the local RTC.
        local_rtc: bool,
        /// Whether to fix the system time.
        fix_system: bool,
        /// Whether the operation is interactive.
        interactive: bool,
        /// Channel to send the result of the operation.
        reply_to: oneshot::Sender<Result<(), TimeDateError>>,
    },
    SetNTP {
        /// Whether to use NTP.
        use_ntp: bool,
        /// Whether the operation is interactive.
        interactive: bool,
        /// Channel to send the result of the operation.
        reply_to: oneshot::Sender<Result<(), TimeDateError>>,
    },
    SetTime {
        /// The time to set in microseconds since the epoch.
        usec_utc: i64,
        /// Whether the time is relative to the current time.
        relative: bool,
        /// Whether the operation is interactive.
        interactive: bool,
        /// Channel to send the result of the operation.
        reply_to: oneshot::Sender<Result<(), TimeDateError>>,
    },
    GetTimeUSec {
        /// Channel to send the result of the operation.
        reply_to: oneshot::Sender<Result<u64, TimeDateError>>,
    },
}


/// A client for interacting with the systemd-timedate D-Bus service.
///
/// This client provides methods to manage system time and timezone settings
/// through D-Bus communication with the systemd-timedate service.
pub struct TimeDateClient {
    cn: Connection,
}


impl TimeDateClient {
    /// Creates a new instance of TimeDateClient.
    ///
    /// Establishes a connection to the system D-Bus and returns a new TimeDateClient instance.
    ///
    /// # Errors
    ///
    /// Returns a `TimeDateError::InitBusError` if the D-Bus connection cannot be established.
    pub async fn new() -> Result<Self, TimeDateError> {
        let cn = Connection::system().await.map_err(|e| {
            TimeDateError::InitBusError(format!("{}", e)) //Dbus_connection_error
        })?;
        Ok(Self { cn })
    }
    /// Runs the timedate client event loop.
    ///
    /// Processes incoming timedate requests received through the provided channel.
    /// This method runs indefinitely until the channel is closed or an error occurs.
    ///
    /// # Arguments
    ///
    /// * `request` - A channel receiver for `TimeDateRequest` messages
    ///
    /// # Errors
    ///
    /// Returns a `TimeDateError::CreateProxyError` if the D-Bus proxy cannot be created.
    pub async fn run(
        &mut self,
        mut request: mpsc::Receiver<TimeDateRequest>,
    ) -> Result<(), TimeDateError> {
        info!("init run handler");
        let proxy = match timedate1Proxy::new(&self.cn).await {
            Ok(n) => n,
            Err(e) => {
                error!("failed to create Bluez proxy: {}", e);
                return Err(TimeDateError::CreateProxyError(format!("{}", e)));
            }
        };
        let service = TimeDateService::new(proxy);
        loop {
            select! {
                msg = request.recv() => {
                    if let Some(request) = msg {
                        match request {
                            TimeDateRequest::ListTimezones { reply_to } => {
                                if let Err(err) = reply_to.send(service.list_time_zones().await) {
                                    error!("failed to send ListTimezones response: {:?}", err);
                                }
                            }
                            TimeDateRequest::SetTimezone { timezone, interactive, reply_to } => {
                                if let Err(err) = reply_to.send(service.set_timezone(&timezone, interactive).await) {
                                    error!("failed to send SetTimezone response: {:?}", err);
                                }
                            }
                            TimeDateRequest::GetTimeZone { reply_to } => {
                                if let Err(err) = reply_to.send(service.get_timezone().await) {
                                    error!("failed to send SetTimezone response: {:?}", err);
                                }
                            }
                            TimeDateRequest::SetLocalRTC { local_rtc, fix_system, interactive, reply_to } => {
                                if let Err(err) = reply_to.send(service.set_local_rtc(local_rtc, fix_system, interactive).await) {
                                    error!("failed to send SetLocalRTC response: {:?}", err);
                                }
                            }
                            TimeDateRequest::SetNTP { use_ntp, interactive, reply_to } => {
                                if let Err(err) = reply_to.send(service.set_ntp(use_ntp, interactive).await) {
                                    error!("failed to send SetNTP response: {:?}", err);
                                }
                            }
                            TimeDateRequest::SetTime { usec_utc, relative, interactive, reply_to } => {
                                if let Err(err) = reply_to.send(service.set_time(usec_utc, relative, interactive).await) {
                                    error!("failed to send SetTime response: {:?}", err);
                                }
                            }
                            TimeDateRequest::GetTimeUSec { reply_to } => {
                                if let Err(err) = reply_to.send(service.get_time_usec().await) {
                                    error!("failed to send GetTimeUSec response: {:?}", err);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
