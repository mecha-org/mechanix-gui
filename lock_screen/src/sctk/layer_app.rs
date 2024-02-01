use crate::input::{keyboard, pointer};
use crate::lock_screen::Message;
use crate::sctk::layer_surface::LayerSurfaceContainer;
use anyhow::Context;
use iced::mouse::ScrollDelta;
use iced::{Event, Theme};
use iced_wgpu::wgpu::{self, Backends};
use smithay_client_toolkit::compositor::{CompositorHandler, CompositorState};
use smithay_client_toolkit::output::{OutputHandler, OutputState};
use smithay_client_toolkit::reexports::calloop::channel::Sender;
use smithay_client_toolkit::reexports::calloop::timer::{TimeoutAction, Timer};
use smithay_client_toolkit::reexports::calloop::{self, EventLoop};
use smithay_client_toolkit::reexports::calloop_wayland_source::WaylandSource;
use smithay_client_toolkit::reexports::client::globals::registry_queue_init;
use smithay_client_toolkit::reexports::client::protocol::wl_keyboard::{self, WlKeyboard};
use smithay_client_toolkit::reexports::client::protocol::wl_output::{self, WlOutput};
use smithay_client_toolkit::reexports::client::protocol::wl_pointer::{
    self, AxisSource, WlPointer,
};
use smithay_client_toolkit::reexports::client::protocol::wl_seat::WlSeat;
use smithay_client_toolkit::reexports::client::protocol::wl_surface::WlSurface;
use smithay_client_toolkit::reexports::client::{
    Connection, Dispatch, EventQueue, Proxy, QueueHandle,
};
use smithay_client_toolkit::registry::{ProvidesRegistryState, RegistryState};
use smithay_client_toolkit::seat::keyboard::{KeyEvent, KeyboardHandler, Keysym, Modifiers};
use smithay_client_toolkit::seat::pointer::{PointerEvent, PointerEventKind, PointerHandler};
use smithay_client_toolkit::seat::{Capability, SeatHandler, SeatState};
use smithay_client_toolkit::{
    delegate_compositor, delegate_keyboard, delegate_output, delegate_pointer, delegate_registry,
    delegate_seat, registry_handlers,
};
use std::cell::RefCell;
use std::time::Duration;
use std::{cell::OnceCell, rc::Rc};
use tracing::debug;
use wayland_protocols::ext::session_lock::v1::client::ext_session_lock_manager_v1::ExtSessionLockManagerV1;
use wayland_protocols::ext::session_lock::v1::client::ext_session_lock_surface_v1::{
    self, ExtSessionLockSurfaceV1,
};
use wayland_protocols::ext::session_lock::v1::client::ext_session_lock_v1::{
    self, ExtSessionLockV1,
};

#[derive(Debug)]
pub enum LockMessage {
    Unlock,
}

pub struct LayerSurfaceApp {
    pub registry_state: RegistryState,
    pub seat_state: SeatState,
    pub output_state: OutputState,

    container: Option<LayerSurfaceContainer>,
    conn: Connection,
    instance: wgpu::Instance,
    adapter: wgpu::Adapter,
    wl_surface: WlSurface,
    device: Rc<wgpu::Device>,
    queue: Rc<wgpu::Queue>,
    // loop_handle: LoopHandle<'static, Self>,

    // input
    pub keyboard: Option<wl_keyboard::WlKeyboard>,
    pub keyboard_focus: bool,
    pub keyboard_modifiers: Modifiers,

    pub pointer: Option<wl_pointer::WlPointer>,
    pub pointer_location: (f64, f64),

    // session lock
    session_lock_manager: ExtSessionLockManagerV1,
    session_lock: ExtSessionLockV1,
    session_lock_surface: ExtSessionLockSurfaceV1,
    session_lock_configure_serial: Option<u32>,
    queue_handle: QueueHandle<LayerSurfaceApp>,

    // configure
    initial_configure_sent: bool,

    // channel
    channel_tx: Sender<LockMessage>,
}

impl LayerSurfaceApp {
    pub fn new() -> anyhow::Result<(Self, EventLoop<'static, Self>)> {
        let conn = Connection::connect_to_env()?;
        let (globals, event_queue) =
            registry_queue_init::<Self>(&conn).context("failed to init registry queue")?;

        let queue_handle = event_queue.handle();

        let compositor = CompositorState::bind(&globals, &queue_handle)
            .context("wl_compositor not availible")?;
        let session_lock_manager = globals
            .bind::<ExtSessionLockManagerV1, _, _>(
                &queue_handle,
                core::ops::RangeInclusive::new(1, 1),
                (),
            )
            .map_err(|_| "compositor does not implement ext session lock manager (v1).")
            .unwrap();
        let output_state = OutputState::new(&globals, &queue_handle);

        let wl_surface = compositor.create_surface(&queue_handle);
        let session_lock = session_lock_manager.lock(&queue_handle, ());
        let output = output_state.outputs().next().unwrap();
        // set surface role as session lock surface
        let session_lock_surface =
            session_lock.get_lock_surface(&wl_surface, &output, &queue_handle, ());

        println!("locked surface {:?}", session_lock_surface);

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::GL | wgpu::Backends::VULKAN,
            ..Default::default()
        });

        let adapter = futures::executor::block_on(async {
            wgpu::util::initialize_adapter_from_env_or_default(
                &instance,
                Backends::GL | Backends::VULKAN,
                None,
            )
            .await
            .unwrap()
        });

        let (device, queue) = futures::executor::block_on(async {
            let adapter_features = adapter.features();
            let needed_limits = wgpu::Limits::default();

            adapter
                .request_device(
                    &wgpu::DeviceDescriptor {
                        label: None,
                        features: adapter_features & wgpu::Features::default(),
                        limits: needed_limits,
                    },
                    None,
                )
                .await
                .expect("Request device")
        });

        let event_loop = EventLoop::<LayerSurfaceApp>::try_new()?;
        let loop_handle = event_loop.handle();
        debug!("create wayland source");

        WaylandSource::new(conn.clone(), event_queue)
            .insert(loop_handle.clone())
            .expect("failed to insert wayland source into event loop");

        // subscribe to events channel
        let (channel_tx, channel_rx) = calloop::channel::channel();

        let _ = loop_handle.insert_source(channel_rx, |event, _, app| {
            let _ = match event {
                calloop::channel::Event::Msg(lock_msg) => {
                    let _ = match lock_msg {
                        LockMessage::Unlock => {
                            app.session_lock.unlock_and_destroy();
                            app.session_lock_surface.destroy();
                            app.session_lock.destroy();
                            app.session_lock_manager.destroy();
                            app.conn
                                .display()
                                .sync(&app.queue_handle, app.wl_surface.clone());
                            std::thread::sleep(std::time::Duration::from_millis(100));
                            std::process::exit(0);
                        }
                    };
                }
                calloop::channel::Event::Closed => {}
            };
        });

        // add timer
        let source = Timer::from_duration(std::time::Duration::from_secs(2));
        loop_handle
            .insert_source(
                // a type which implements the EventSource trait
                source,
                // a callback that is invoked whenever this source generates an event
                |event, _metadata, app| {
                    println!("Timeout for {:?} expired!", event);
                    // The timer event source requires us to return a TimeoutAction to
                    // specify if the timer should be rescheduled. In our case we just drop it.
                    // let container = /
                    let timeout_action = TimeoutAction::ToDuration(Duration::from_millis(50));

                    if app.container.is_none() {
                        return timeout_action;
                    }

                    let container = app.container.as_mut().unwrap();

                    container
                        .iced_program
                        .queue_message(Message::Tick(time::OffsetDateTime::now_utc()));
                    container.update_iced_app(app.pointer_location);
                    timeout_action
                },
            )
            .expect("Failed to insert event source!");

        Ok((
            LayerSurfaceApp {
                output_state,
                seat_state: SeatState::new(&globals, &queue_handle),
                registry_state: RegistryState::new(&globals),
                queue_handle,
                container: None,
                conn,
                wl_surface: wl_surface.clone(),
                instance,
                adapter,
                device: Rc::new(device),
                queue: Rc::new(queue),

                // input
                keyboard: None,
                keyboard_focus: false,
                keyboard_modifiers: Modifiers::default(),

                pointer: None,
                pointer_location: (0.0, 0.0),

                // session
                session_lock_manager: session_lock_manager,
                session_lock: session_lock,
                session_lock_surface: session_lock_surface,
                session_lock_configure_serial: None,
                // loop_handle: event_loop.handle(),

                // configure
                initial_configure_sent: false,

                // channel
                channel_tx,
            },
            event_loop,
        ))
    }

    pub fn run(&mut self, (width, height): (u32, u32)) {
        if let Ok(mut container) = LayerSurfaceContainer::new(
            &self.conn,
            &self.wl_surface,
            &self.instance,
            &self.adapter,
            self.device.clone(),
            self.queue.clone(),
            (width, height),
            self.channel_tx.clone(),
        ) {
            container.update_iced_app(self.pointer_location);
            if !self.initial_configure_sent {
                self.initial_configure_sent = true;
                container.draw(&self.queue_handle, &self.wl_surface);
            }
            self.container = Some(container);
        }
    }

    pub fn send_event(&mut self, event: Event) {
        let container = self.container.as_mut().unwrap();
        container.queue_event(event);
    }

    pub fn configure_wgpu_surfaces(&self) {
        let container = self.container.as_ref().unwrap();
        container.configure_wgpu_surface(&self.device);
    }

    pub fn dispatch_loops(&mut self) -> anyhow::Result<()> {
        if self.container.is_none() {
            return Ok(());
        }

        // let container = self.container.as_mut().unwrap();

        // let res = event_loop.dispatch(Duration::ZERO, container);
        // println!("error is {}", res.is_err());
        Ok(())
    }
}

delegate_compositor!(LayerSurfaceApp);
delegate_output!(LayerSurfaceApp);
delegate_registry!(LayerSurfaceApp);
delegate_seat!(LayerSurfaceApp);
delegate_keyboard!(LayerSurfaceApp);
delegate_pointer!(LayerSurfaceApp);

impl ProvidesRegistryState for LayerSurfaceApp {
    fn registry(&mut self) -> &mut RegistryState {
        &mut self.registry_state
    }

    registry_handlers!(OutputState);
}

impl CompositorHandler for LayerSurfaceApp {
    fn scale_factor_changed(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _surface: &WlSurface,
        _new_factor: i32,
    ) {
    }

    fn frame(
        &mut self,
        _conn: &Connection,
        qh: &QueueHandle<Self>,
        surface: &WlSurface,
        _time: u32,
    ) {
        let container = self.container.as_mut().unwrap();
        container.update_iced_app(self.pointer_location);
        container.draw(qh, surface);
        // container.draw(qh, surface);
    }

    fn transform_changed(
        &mut self,
        _: &Connection,
        _: &QueueHandle<Self>,
        _: &WlSurface,
        _: wl_output::Transform,
    ) {
        todo!()
    }
}

impl OutputHandler for LayerSurfaceApp {
    fn output_state(&mut self) -> &mut OutputState {
        &mut self.output_state
    }

    fn new_output(&mut self, _: &Connection, _: &QueueHandle<Self>, _: WlOutput) {}

    fn update_output(&mut self, _: &Connection, _: &QueueHandle<Self>, _: WlOutput) {}

    fn output_destroyed(&mut self, _: &Connection, _: &QueueHandle<Self>, _: WlOutput) {}
}

impl SeatHandler for LayerSurfaceApp {
    fn seat_state(&mut self) -> &mut SeatState {
        &mut self.seat_state
    }

    fn new_seat(&mut self, _conn: &Connection, _qh: &QueueHandle<Self>, _seat: WlSeat) {}

    fn new_capability(
        &mut self,
        _conn: &Connection,
        qh: &QueueHandle<Self>,
        seat: WlSeat,
        capability: Capability,
    ) {
        if capability == Capability::Keyboard && self.keyboard.is_none() {
            let keyboard = self.seat_state.get_keyboard(qh, &seat, None).unwrap();
            self.keyboard = Some(keyboard);
        }
        if capability == Capability::Pointer && self.pointer.is_none() {
            let pointer = self.seat_state.get_pointer(qh, &seat).unwrap();
            self.pointer = Some(pointer);
        }
    }

    fn remove_capability(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _seat: WlSeat,
        capability: Capability,
    ) {
        if capability == Capability::Keyboard {
            if let Some(keyboard) = self.keyboard.take() {
                keyboard.release();
            }
        }
        if capability == Capability::Pointer {
            if let Some(pointer) = self.pointer.take() {
                pointer.release();
            }
        }
    }

    fn remove_seat(&mut self, _conn: &Connection, _qh: &QueueHandle<Self>, _seat: WlSeat) {}
}

impl KeyboardHandler for LayerSurfaceApp {
    fn enter(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _keyboard: &WlKeyboard,
        surface: &WlSurface,
        _serial: u32,
        _raw: &[u32],
        _keysyms: &[Keysym],
    ) {
        if &self.wl_surface != surface {
            return;
        }

        self.keyboard_focus = true;
    }

    fn leave(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _keyboard: &WlKeyboard,
        surface: &WlSurface,
        _serial: u32,
    ) {
        if &self.wl_surface != surface {
            return;
        }

        self.keyboard_focus = false;
    }

    fn press_key(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _keyboard: &WlKeyboard,
        _serial: u32,
        event: KeyEvent,
    ) {
        debug!("start of press_key");
        if !self.keyboard_focus {
            return;
        }

        let Some(keycode) = keyboard::keysym_to_keycode(event.keysym) else {
            return;
        };

        let mut modifiers = iced_runtime::keyboard::Modifiers::default();

        let Modifiers {
            ctrl,
            alt,
            shift,
            caps_lock: _,
            logo,
            num_lock: _,
        } = &self.keyboard_modifiers;

        if *ctrl {
            modifiers |= iced_runtime::keyboard::Modifiers::CTRL;
        }
        if *alt {
            modifiers |= iced_runtime::keyboard::Modifiers::ALT;
        }
        if *shift {
            modifiers |= iced_runtime::keyboard::Modifiers::SHIFT;
        }
        if *logo {
            modifiers |= iced_runtime::keyboard::Modifiers::LOGO;
        }

        let event = iced::Event::Keyboard(iced_runtime::keyboard::Event::KeyPressed {
            key_code: keycode,
            modifiers,
        });

        self.send_event(event);
    }

    fn release_key(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _keyboard: &WlKeyboard,
        _serial: u32,
        event: KeyEvent,
    ) {
        if !self.keyboard_focus {
            return;
        }

        let Some(keycode) = keyboard::keysym_to_keycode(event.keysym) else {
            return;
        };

        let mut modifiers = iced_runtime::keyboard::Modifiers::default();

        let Modifiers {
            ctrl,
            alt,
            shift,
            caps_lock: _,
            logo,
            num_lock: _,
        } = &self.keyboard_modifiers;

        if *ctrl {
            modifiers |= iced_runtime::keyboard::Modifiers::CTRL;
        }
        if *alt {
            modifiers |= iced_runtime::keyboard::Modifiers::ALT;
        }
        if *shift {
            modifiers |= iced_runtime::keyboard::Modifiers::SHIFT;
        }
        if *logo {
            modifiers |= iced_runtime::keyboard::Modifiers::LOGO;
        }

        let event = iced::Event::Keyboard(iced_runtime::keyboard::Event::KeyReleased {
            key_code: keycode,
            modifiers,
        });

        self.send_event(event);
    }

    fn update_modifiers(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _keyboard: &WlKeyboard,
        _serial: u32,
        modifiers: Modifiers,
    ) {
        self.keyboard_modifiers = modifiers;
    }
}

impl PointerHandler for LayerSurfaceApp {
    fn pointer_frame(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _pointer: &WlPointer,
        events: &[PointerEvent],
    ) {
        for event in events {
            if event.surface != self.wl_surface {
                continue;
            }

            let iced_event = match event.kind {
                PointerEventKind::Enter { .. } => {
                    iced::Event::Mouse(iced::mouse::Event::CursorEntered)
                }
                PointerEventKind::Leave { .. } => {
                    iced::Event::Mouse(iced::mouse::Event::CursorLeft)
                }
                PointerEventKind::Motion { .. } => {
                    self.pointer_location = event.position;
                    iced::Event::Mouse(iced::mouse::Event::CursorMoved {
                        position: iced::Point {
                            x: event.position.0 as f32,
                            y: event.position.1 as f32,
                        },
                    })
                }
                PointerEventKind::Press { button, .. } => {
                    println!("pointer_press");

                    if let Some(button) = pointer::button_to_iced_button(button) {
                        iced::Event::Mouse(iced::mouse::Event::ButtonPressed(button))
                    } else {
                        continue;
                    }
                }
                PointerEventKind::Release { button, .. } => {
                    if let Some(button) = pointer::button_to_iced_button(button) {
                        iced::Event::Mouse(iced::mouse::Event::ButtonReleased(button))
                    } else {
                        continue;
                    }
                }
                PointerEventKind::Axis {
                    horizontal,
                    vertical,
                    source,
                    time: _,
                } => {
                    let delta = match source.unwrap() {
                        AxisSource::Wheel => ScrollDelta::Lines {
                            x: horizontal.discrete as f32,
                            y: vertical.discrete as f32,
                        },
                        AxisSource::Finger => ScrollDelta::Pixels {
                            x: horizontal.absolute as f32,
                            y: vertical.absolute as f32,
                        },
                        AxisSource::Continuous => ScrollDelta::Pixels {
                            x: horizontal.absolute as f32,
                            y: vertical.absolute as f32,
                        },
                        AxisSource::WheelTilt => ScrollDelta::Lines {
                            x: horizontal.discrete as f32,
                            y: vertical.discrete as f32,
                        },
                        _ => continue,
                    };
                    iced::Event::Mouse(iced::mouse::Event::WheelScrolled { delta })
                }
            };

            self.send_event(iced_event);
        }
    }
}

impl Dispatch<ExtSessionLockManagerV1, ()> for LayerSurfaceApp {
    fn event(
        _: &mut Self,
        _: &ExtSessionLockManagerV1,
        event: <ExtSessionLockManagerV1 as Proxy>::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
        println!("event::ext_session_lock_manager_v1 {:?}", event);
    }
}

impl Dispatch<ExtSessionLockV1, ()> for LayerSurfaceApp {
    fn event(
        state: &mut Self,
        _: &ExtSessionLockV1,
        event: <ExtSessionLockV1 as Proxy>::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
        match event {
            ext_session_lock_v1::Event::Locked => {
                println!("event::ext_session_lock_v1;;locked");
            }
            ext_session_lock_v1::Event::Finished => {
                println!("event::ext_session_lock_v1::finished");
            }
            _ => {}
        }
    }
}

impl Dispatch<ExtSessionLockSurfaceV1, ()> for LayerSurfaceApp {
    fn event(
        state: &mut Self,
        surface: &ExtSessionLockSurfaceV1,
        event: <ExtSessionLockSurfaceV1 as Proxy>::Event,
        _: &(),
        _: &Connection,
        qh: &QueueHandle<Self>,
    ) {
        println!("ext_session_lock_surface_v1::event {:?}", event);
        state.session_lock_surface = surface.to_owned();
        match event {
            ext_session_lock_surface_v1::Event::Configure {
                serial,
                width,
                height,
            } => {
                println!(
                    "event::ext_session_lock_surface_v1::configure {} {} {}",
                    serial, width, height
                );

                debug!("update_iced_app");
                state.session_lock_surface.ack_configure(serial);

                println!("creating app");
                state.run((width, height));

                // state.update_iced_app();

                // if !state.initial_configure_sent {
                //     state.initial_configure_sent = true;
                //     state.draw(qh, &state.wl_surface.clone());
                // }

                // // state.session_lock_configure_serial = Some(serial);
            }
            _ => todo!(),
        }
    }
}
