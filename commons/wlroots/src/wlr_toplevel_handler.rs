use anyhow::{bail, Error, Result};
use async_trait::async_trait;
use std::collections::HashMap;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::{mpsc, oneshot};
use tokio::{select, time};
use tracing::{debug, info};
use wayrs_client::global::GlobalsExt;
use wayrs_client::{Connection, EventCtx};
use wayrs_protocols::wlr_foreign_toplevel_management_unstable_v1::*;

use crate::errors::{WlrootsError, WlrootsErrorCodes};

#[derive(Default, Debug, Clone)]
pub struct Toplevel {
    title: Option<String>,
    app_id: Option<String>,
    state: Option<Vec<u8>>,
    is_active: bool,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ServiceStatus {
    INACTIVE = 0,
    STARTED = 1,
    STOPPED = -1,
}

#[async_trait]
pub trait ServiceHandler {
    fn get_status(&self) -> Result<ServiceStatus>;
    fn is_stopped(&self) -> Result<bool>;
    fn is_started(&self) -> Result<bool>;
    async fn start(&mut self) -> Result<bool>;
    async fn stop(&mut self) -> Result<bool>;
}

struct ConnectionContext {
    toplevel_event_tx: Sender<WlrToplevelEvent>,
    conn_error: Option<Error>,
    active_toplevel_title: Option<String>,
    active_toplevel_app_id: Option<String>,
    toplevels: HashMap<ZwlrForeignToplevelHandleV1, Toplevel>,
    active_toplevel: Option<ZwlrForeignToplevelHandleV1>,
}

pub struct WlrToplevelHandler {
    status: ServiceStatus,
    state: ConnectionContext,
}

pub enum WlrToplevelHandlerMessage {
    GetActiveToplevelTitle {
        reply_to: oneshot::Sender<Result<Option<String>>>,
    },
    GetActiveToplevelAppId {
        reply_to: oneshot::Sender<Result<Option<String>>>,
    },
    GetToplevels {
        reply_to: oneshot::Sender<Result<Vec<Toplevel>>>,
    },
    GetActiveToplevel {
        reply_to: oneshot::Sender<Result<Option<Toplevel>>>,
    },
    MinimizeAllTopLevel {
        reply_to: oneshot::Sender<Result<Option<bool>>>,
    },
}

#[derive(Debug)]
pub enum WlrToplevelEvent {
    ToplevelCreated,
    ToplevelActive,
    ToplevelClosed,
    ToplevelInactive,
}

pub struct WlrToplevelHandlerOptions {
    pub toplevel_event_tx: Sender<WlrToplevelEvent>,
}

impl WlrToplevelHandler {
    pub fn new(options: WlrToplevelHandlerOptions) -> Self {
        Self {
            status: ServiceStatus::STARTED,
            state: ConnectionContext {
                conn_error: None,
                toplevel_event_tx: options.toplevel_event_tx,
                toplevels: HashMap::new(),
                active_toplevel_title: Some(String::new()),
                active_toplevel_app_id: Some(String::new()),
                active_toplevel: None,
            },
        }
    }

    pub async fn run(
        &mut self,
        mut toplevel_message_rx: Receiver<WlrToplevelHandlerMessage>,
    ) -> Result<()> {
        let target = "wlr_toplevel_handler";
        let task = "run";
        // create new connection
        let (mut conn, globals) = match Connection::async_connect_and_collect_globals().await {
            Ok((c, g)) => (c, g),
            Err(err) => bail!(WlrootsError::new(
                WlrootsErrorCodes::ConnectWaylandServerError,
                format!("error connecting to the wayland server, internal - {}", err),
                true
            )),
        };

        // setup the callback
        let _ = match globals
            .bind_with_cb(&mut conn, 1..=3, toplevel_manager_cb) {
                Ok(x) => x,
                Err(err) => bail!(WlrootsError::new(
                    WlrootsErrorCodes::WaylandCompositorSupportError,
                    format!("wayland compositor does not support wlr-foreign-toplevel-management [v1.0-v3.0], internal - {}", err),
                    true
                )),
        };

        // flush event queue
        let _ = match conn.async_flush().await {
            Ok(x) => x,
            Err(err) => bail!(WlrootsError::new(
                WlrootsErrorCodes::WaylandServerFlushingError,
                format!(
                    "error from wayland server in flushing event queue, internal - {}",
                    err
                ),
                true
            )),
        };

        // wait for messages
        loop {
            select! {
                _ = conn.async_recv_events() => {
                    info!(target, task,"foreign_toplevel_manager_v1:message");
                    conn.dispatch_events(&mut self.state);
                    if let Some(err) = self.state.conn_error.take() {
                        bail!(WlrootsError::new(
                            WlrootsErrorCodes::ToplevelManagerFinishedError,
                            format!(
                                "foreign toplevel manager is finished, internal -{}",
                                err
                            ),
                            true
                        ))

                    }


                    let _ = match conn.async_flush().await {
                        Ok(x) => x,
                        Err(err) => bail!(WlrootsError::new(
                            WlrootsErrorCodes::WaylandServerFlushingError,
                            format!(
                                "error from wayland server in flushing event queue, internal - {}",
                                err
                            ),
                            true
                        ))
                    };
                }
                msg = toplevel_message_rx.recv() => {
                    if msg.is_none() {
                        continue;
                    }

                    match msg {
                        Some(_) => {
                            let _ = match msg.unwrap() {
                                WlrToplevelHandlerMessage::GetActiveToplevelTitle { reply_to } => {
                                    let active_title = self.get_active_toplevel_title();
                                    let _ = reply_to.send(active_title);
                                },
                                WlrToplevelHandlerMessage::GetActiveToplevelAppId { reply_to } => {
                                    let active_app_id = self.get_active_toplevel_app_id();
                                    let _ = reply_to.send(active_app_id);
                                },
                                WlrToplevelHandlerMessage::GetToplevels { reply_to } => {
                                    let top_levels = self.get_toplevels();
                                    let _ = reply_to.send(top_levels);
                                },
                                WlrToplevelHandlerMessage::GetActiveToplevel { reply_to } => {
                                    let top_level = self.get_active_toplevel();
                                    let _ = reply_to.send(top_level);
                                },
                                WlrToplevelHandlerMessage::MinimizeAllTopLevel { reply_to } => {
                                    let success: bool =
                                    match self.get_toplevels_map() {
                                        Ok(top_levels) => {
                                            top_levels.into_iter().for_each(
                                                |(top_level_handle, top_level)| match &top_level.state {
                                                    Some(state) => {
                                                        let is_minimized = state
                                                            .chunks_exact(4)
                                                            .map(|b| u32::from_ne_bytes(b.try_into().unwrap()))
                                                            .any(|s| {
                                                                s == zwlr_foreign_toplevel_handle_v1::State::Minimized
                                                                    as u32
                                                            });
                                                        if !is_minimized{
                                                            top_level_handle.set_minimized(&mut conn);
                                                        }
                                                    }
                                                    None => {
                                                        debug!("top level apps not found ");
                                                    }
                                                },
                                            );
                                            true
                                        }
                                        Err(_) => false,
                                    };
                                    let _ = reply_to.send(Ok(Some(success)));
                                }
                            };
                        }
                        None => ()
                    };
                }
            }
        }
    }
}

impl WlrToplevelHandler {
    pub fn get_active_toplevel_title(&self) -> Result<Option<String>> {
        let active_title = &self.state.active_toplevel_title;
        let conn_err = &self.state.conn_error;

        if conn_err.is_some() {
            bail!(WlrootsError::new(
                WlrootsErrorCodes::ConnectionError,
                format!("wayland connection error in get active toplevel title",),
                true
            ));
        }
        Ok(active_title.clone())
    }

    pub fn get_active_toplevel_app_id(&self) -> Result<Option<String>> {
        let active_app_id = &self.state.active_toplevel_app_id;
        let conn_err = &self.state.conn_error;

        if conn_err.is_some() {
            bail!(WlrootsError::new(
                WlrootsErrorCodes::ConnectionError,
                format!("wayland connection error in get active toplevel app id",),
                true
            ));
        }
        Ok(active_app_id.clone())
    }

    pub fn get_toplevels(&self) -> Result<Vec<Toplevel>> {
        let toplevel_map = &self.state.toplevels;
        let conn_err = &self.state.conn_error;

        if conn_err.is_some() {
            bail!(WlrootsError::new(
                WlrootsErrorCodes::ConnectionError,
                format!("wayland connection error in get toplevels",),
                true
            ));
        }

        Ok(toplevel_map.values().cloned().collect::<Vec<Toplevel>>())
    }

    pub fn get_toplevels_map(&self) -> Result<&HashMap<ZwlrForeignToplevelHandleV1, Toplevel>> {
        let toplevel_map: &HashMap<ZwlrForeignToplevelHandleV1, Toplevel> = &self.state.toplevels;
        let conn_err = &self.state.conn_error;

        if conn_err.is_some() {
            bail!(WlrootsError::new(
                WlrootsErrorCodes::ConnectionError,
                format!("wayland connection error in get toplevels",),
                true
            ));
        }

        Ok(toplevel_map)
    }

    pub fn get_active_toplevel(&self) -> Result<Option<Toplevel>> {
        let toplevel_map = &self.state.toplevels;
        let conn_err = &self.state.conn_error;

        if conn_err.is_some() {
            bail!(WlrootsError::new(
                WlrootsErrorCodes::ConnectionError,
                format!("wayland connection error in get active toplevel",),
                true
            ));
        }

        let active_toplevel = toplevel_map.values().cloned().find(|t| t.is_active);
        Ok(active_toplevel)
    }
}

#[async_trait]
impl ServiceHandler for WlrToplevelHandler {
    async fn start(&mut self) -> Result<bool> {
        self.status = ServiceStatus::STARTED;
        Ok(true)
    }

    async fn stop(&mut self) -> Result<bool> {
        self.status = ServiceStatus::STOPPED;
        Ok(true)
    }

    fn get_status(&self) -> anyhow::Result<ServiceStatus> {
        Ok(self.status)
    }

    fn is_stopped(&self) -> Result<bool> {
        Ok(self.status == ServiceStatus::STOPPED)
    }

    fn is_started(&self) -> Result<bool> {
        Ok(self.status == ServiceStatus::STARTED)
    }
}

fn toplevel_manager_cb(ctx: EventCtx<ConnectionContext, ZwlrForeignToplevelManagerV1>) {
    let target = "wlr_toplevel_handler";
    let task = "toplevel_manager_cb";
    use zwlr_foreign_toplevel_manager_v1::Event;
    match ctx.event {
        Event::Toplevel(toplevel) => {
            info!(
                target,
                task, "foreign_toplevel_manager_v1:toplevel {:?}", toplevel
            );
            //toplevel.set_fullscreen(ctx.conn, None);
            let toplevel_event_tx = ctx.state.toplevel_event_tx.clone();
            tokio::task::spawn(async move {
                let _ = toplevel_event_tx
                    .send(WlrToplevelEvent::ToplevelCreated)
                    .await;
            });
            ctx.state.toplevels.insert(toplevel, Toplevel::default());
            ctx.conn.set_callback_for(toplevel, toplevel_cb);
        }
        Event::Finished => {
            info!(target, task, "foreign_toplevel_manager_v1:finished");
            ctx.state.conn_error = Some(Error::msg(String::from("unexpected 'finished' event")));
            ctx.conn.break_dispatch_loop();
        }
        _ => (),
    }
}

fn toplevel_cb(ctx: EventCtx<ConnectionContext, ZwlrForeignToplevelHandleV1>) {
    let target = "wlr_toplevel_handler";
    let task = "toplevel_cb";
    use zwlr_foreign_toplevel_handle_v1::Event;
    let toplevel_event_tx = ctx.state.toplevel_event_tx.clone();
    let Some(toplevel) = ctx.state.toplevels.get_mut(&ctx.proxy) else {
        return;
    };

    let send_toplevel_event_tx_message = |event: WlrToplevelEvent| {
        tokio::task::spawn(async move {
            let _ = toplevel_event_tx.send(event).await;
        });
    };

    info!(target, task, "foreign_toplevel_handle_v1:event");

    match ctx.event {
        Event::AppId(app_id) => {
            let app_id_str: String = String::from_utf8_lossy(app_id.as_bytes()).into();
            toplevel.app_id = Some(app_id_str.clone());
            info!(
                target,
                task,
                "foreign_toplevel_handle_v1:app_id proxy={:?} app_id={:?}",
                ctx.proxy,
                app_id_str
            );
        }
        Event::Title(title) => {
            let title_str: String = String::from_utf8_lossy(title.as_bytes()).into();
            toplevel.title = Some(title_str.clone());
            info!(
                target,
                task,
                "foreign_toplevel_handle_v1:title proxy={:?} title={:?}",
                ctx.proxy,
                title_str
            );
        }
        Event::State(state) => {
            let title_str = match toplevel.title.clone() {
                Some(v) => v,
                None => String::new(),
            };
            info!(
                target,
                task,
                "foreign_toplevel_handle_v1:state proxy={:?} title={} state={:?}",
                ctx.proxy,
                title_str,
                state
            );

            let is_active = state
                .chunks_exact(4)
                .map(|b| u32::from_ne_bytes(b.try_into().unwrap()))
                .any(|s| s == zwlr_foreign_toplevel_handle_v1::State::Activated as u32);

            // let is_minimized = state
            // .chunks_exact(4)
            // .map(|b| u32::from_ne_bytes(b.try_into().unwrap()))
            // .any(|s| s == zwlr_foreign_toplevel_handle_v1::State::Minimized as u32);

            if is_active {
                send_toplevel_event_tx_message(WlrToplevelEvent::ToplevelActive);
            }
            toplevel.state = Some(state);
            toplevel.is_active = is_active;
        }
        Event::Closed => {
            let title_str = match toplevel.title.clone() {
                Some(v) => v,
                None => String::new(),
            };

            info!(
                target,
                task,
                "zwlr_foreign_toplevel_handle_v1:closed proxy={:?} title={}",
                ctx.proxy,
                title_str
            );
            if ctx.state.active_toplevel == Some(ctx.proxy) {
                ctx.state.active_toplevel = None;
                ctx.state.active_toplevel_title = Some(String::new());
                ctx.state.active_toplevel_app_id = Some(String::new());
            }

            ctx.proxy.destroy(ctx.conn);
            ctx.state.toplevels.remove(&ctx.proxy);
            send_toplevel_event_tx_message.clone()(WlrToplevelEvent::ToplevelClosed);
        }
        Event::Done => {
            let title_str = match toplevel.title.clone() {
                Some(v) => v,
                None => String::new(),
            };

            info!(
                target,
                task,
                "zwlr_foreign_toplevel_handle_v1:done proxy={:?} title={}",
                ctx.proxy,
                title_str
            );
            if toplevel.is_active {
                ctx.state.active_toplevel = Some(ctx.proxy);
                ctx.state.active_toplevel_title = toplevel.title.clone();
                ctx.state.active_toplevel_app_id = toplevel.app_id.clone();

                send_toplevel_event_tx_message.clone()(WlrToplevelEvent::ToplevelActive);
            } else if ctx.state.active_toplevel == Some(ctx.proxy) {
                ctx.state.active_toplevel = None;
                ctx.state.active_toplevel_title = Some(String::new());
                ctx.state.active_toplevel_app_id = Some(String::new());
                send_toplevel_event_tx_message.clone()(WlrToplevelEvent::ToplevelInactive);
            }
        }
        _ => (),
    }
}
