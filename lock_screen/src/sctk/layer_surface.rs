use anyhow::Context;
use iced::{mouse::ScrollDelta, Color, Event, Size, Theme};
use iced_runtime::Debug;
use iced_wgpu::{
    graphics::{Renderer, Viewport},
    wgpu::{self, Backends},
};
use raw_window_handle::{
    RawDisplayHandle, RawWindowHandle, WaylandDisplayHandle, WaylandWindowHandle,
};
use smithay_client_toolkit::{
    compositor::{CompositorHandler, CompositorState},
    delegate_compositor, delegate_keyboard, delegate_layer, delegate_output, delegate_pointer,
    delegate_registry, delegate_seat,
    output::{OutputHandler, OutputState},
    reexports::{
        calloop::{
            channel::Sender,
            timer::{TimeoutAction, Timer},
            EventLoop,
        },
        calloop_wayland_source::WaylandSource,
        client::{
            globals::registry_queue_init,
            protocol::{
                wl_keyboard::{self, WlKeyboard},
                wl_output::{self, WlOutput},
                wl_pointer::{self, AxisSource, WlPointer},
                wl_seat::WlSeat,
                wl_subcompositor,
                wl_surface::WlSurface,
            },
            Connection, Dispatch, EventQueue, Proxy, QueueHandle,
        },
    },
    registry::{ProvidesRegistryState, RegistryState},
    registry_handlers,
    seat::{
        keyboard::{KeyEvent, KeyboardHandler, Keysym, Modifiers},
        pointer::{PointerEvent, PointerEventKind, PointerHandler},
        Capability, SeatHandler, SeatState,
    },
    shell::wlr_layer::Anchor,
};
use std::{
    cell::{OnceCell, RefCell},
    rc::Rc,
    time::Duration,
};
use tracing::{debug, trace};
use wayland_protocols::ext::session_lock::v1::client::{
    ext_session_lock_manager_v1::{self, ExtSessionLockManagerV1},
    ext_session_lock_surface_v1::{self, ExtSessionLockSurfaceV1},
    ext_session_lock_v1::{self, ExtSessionLockV1},
};

use crate::{
    input::{clipboard::WaylandClipboard, keyboard, pointer},
    lock_screen::{LockScreen, Message},
    sctk::raw_handle::RawWaylandHandle,
};

use super::layer_app::{LayerSurfaceApp, LockMessage};

pub struct LayerSurfaceContainer {
    // surface must be dropped before layer
    pub wl_surface: WlSurface,
    pub surface: wgpu::Surface,
    pub dirty: bool,

    pub iced_program: iced_runtime::program::State<LockScreen>,
    // pub layer: LayerSurface,
    pub width: u32,
    pub height: u32,
    pub viewport: Viewport,
    pub capabilities: wgpu::SurfaceCapabilities,

    pub clipboard: WaylandClipboard,

    pub keyboard: Option<wl_keyboard::WlKeyboard>,
    pub keyboard_focus: bool,
    pub keyboard_modifiers: Modifiers,

    pub pointer: Option<wl_pointer::WlPointer>,
    pub pointer_location: (f64, f64),

    pub initial_configure_sent: bool,

    pub device: Rc<wgpu::Device>,
    pub queue: Rc<wgpu::Queue>,
    pub renderer: iced::Renderer,
}

impl LayerSurfaceContainer {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        conn: &Connection,
        wl_surface: &WlSurface,
        instance: &wgpu::Instance,
        adapter: &wgpu::Adapter,
        device: Rc<wgpu::Device>,
        queue: Rc<wgpu::Queue>,
        (width, height): (u32, u32),
        lock_channel: Sender<LockMessage>,
    ) -> anyhow::Result<Self> {
        // let event_loop = EventLoop::<Self>::try_new()?;

        debug!("create wayland handle");
        let wayland_handle = {
            let mut handle = WaylandDisplayHandle::empty();
            handle.display = conn.backend().display_ptr() as *mut _;
            let display_handle = RawDisplayHandle::Wayland(handle);

            let mut handle = WaylandWindowHandle::empty();
            handle.surface = wl_surface.id().as_ptr() as *mut _;
            let window_handle = RawWindowHandle::Wayland(handle);

            RawWaylandHandle(display_handle, window_handle)
        };

        debug!("create wgpu surface");
        let wgpu_surface = unsafe { instance.create_surface(&wayland_handle).unwrap() };

        debug!("get capabilities"); // PERF: SLOW
        let capabilities = wgpu_surface.get_capabilities(adapter);

        debug!("get texture format");
        let format = capabilities
            .formats
            .iter()
            .copied()
            .find(wgpu::TextureFormat::is_srgb)
            .or_else(|| capabilities.formats.first().copied())
            .expect("Get preferred format");

        // TODO: speed up
        debug!("create iced backend"); // PERF: SLOW
        let backend = iced_wgpu::Backend::new(
            &device,
            &queue,
            iced_wgpu::Settings {
                present_mode: wgpu::PresentMode::Mailbox,
                internal_backend: Backends::GL | Backends::VULKAN,
                ..Default::default()
            },
            format,
        );

        debug!("create iced renderer");
        let renderer: Renderer<iced_wgpu::Backend, Theme> = Renderer::new(backend);
        let mut rd = iced::Renderer::Wgpu(renderer);

        let state = {
            iced_runtime::program::State::new(
                LockScreen {
                    lock_channel: Some(lock_channel.clone()),
                    ..Default::default()
                },
                Size::new(width as f32, height as f32),
                &mut rd,
                &mut Debug::new(), // TODO:
            )
        };

        let state = LayerSurfaceContainer {
            iced_program: state,
            wl_surface: wl_surface.clone(),
            // layer,
            width,
            height,
            viewport: Viewport::with_physical_size(Size::new(width, height), 1.0),
            capabilities,

            dirty: true,

            surface: wgpu_surface,

            clipboard: unsafe { WaylandClipboard::new(conn.backend().display_ptr() as *mut _) },

            keyboard: None,
            keyboard_focus: false,
            keyboard_modifiers: Modifiers::default(),

            pointer: None,
            pointer_location: (0.0, 0.0),

            initial_configure_sent: false,

            device: device.clone(),
            queue,
            renderer: rd,
        };

        state.configure_wgpu_surface(&device);

        Ok(state)
    }

    pub fn configure_wgpu_surface(&self, device: &wgpu::Device) {
        let capabilities = &self.capabilities;
        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: capabilities.formats[0],
            width: self.width,
            height: self.height,
            present_mode: wgpu::PresentMode::Mailbox,
            alpha_mode: wgpu::CompositeAlphaMode::PreMultiplied,
            view_formats: vec![capabilities.formats[0]],
        };

        self.surface.configure(device, &surface_config);
    }
}

impl LayerSurfaceContainer {
    pub fn draw(&mut self, queue_handle: &QueueHandle<LayerSurfaceApp>, surface: &WlSurface) {
        tracing::trace!("State::draw");
        if &self.wl_surface != surface {
            println!("surfaces are different");
            return;
        }

        // if !self.dirty {
        //     return;
        // }
        match self.surface.get_current_texture() {
            Ok(frame) => {
                let mut encoder = self
                    .device
                    .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

                let view = frame
                    .texture
                    .create_view(&wgpu::TextureViewDescriptor::default());
                {
                    if let iced::Renderer::Wgpu(renderer) = &mut self.renderer {
                        renderer.with_primitives(|backend, primitives| {
                            backend.present::<String>(
                                &self.device,
                                &self.queue,
                                &mut encoder,
                                Some(iced::Color::new(0.0, 0.0, 0.0, 0.5)),
                                &view,
                                primitives,
                                &self.viewport,
                                &[],
                            );
                        });
                    }
                }

                self.queue.submit(Some(encoder.finish()));
                frame.present();

                surface.damage_buffer(0, 0, self.width as i32, self.height as i32);
                surface.frame(queue_handle, surface.clone());

                surface.commit();
            }
            Err(_) => {
                println!("ERROR");
            }
        }
        self.dirty = false;
    }

    pub fn queue_event(&mut self, event: iced::Event) {
        self.iced_program.queue_event(event);
    }

    pub fn update_iced_app(&mut self, pointer_location: (f64, f64)) {
        let (events, command) = self.iced_program.update(
            self.viewport.logical_size(),
            iced::mouse::Cursor::Available(iced::Point {
                x: pointer_location.0 as f32,
                y: pointer_location.1 as f32,
            }),
            &mut self.renderer,
            &Theme::Dark,
            &iced_wgpu::core::renderer::Style {
                text_color: Color::WHITE,
            },
            &mut self.clipboard,
            &mut Debug::new(),
        );
    }
}
