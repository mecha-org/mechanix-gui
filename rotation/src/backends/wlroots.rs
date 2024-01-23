use tracing::debug;
use wayland_client::{
    event_created_child,
    protocol::{wl_output::Transform, wl_registry},
    Connection, Dispatch, EventQueue, QueueHandle,
};
use wayland_protocols_wlr::output_management::v1::client::{
    zwlr_output_configuration_head_v1::{self, ZwlrOutputConfigurationHeadV1},
    zwlr_output_configuration_v1::{self, ZwlrOutputConfigurationV1},
    zwlr_output_head_v1::{self, ZwlrOutputHeadV1},
    zwlr_output_manager_v1::{self, ZwlrOutputManagerV1},
    zwlr_output_mode_v1::{self, ZwlrOutputModeV1},
};

use super::{DisplayManager, Orientation};

pub struct WaylandBackend {
    state: AppData,
    event_queue: EventQueue<AppData>,
}

impl WaylandBackend {
    pub fn new(target_display: &str) -> Result<WaylandBackend, String> {
        let conn = wayland_client::Connection::connect_to_env()
            .map_err(|_| "Could not connect to wayland socket.")?;
        let wl_display = conn.display();
        let mut event_queue = conn.new_event_queue();
        let _registry = wl_display.get_registry(&event_queue.handle(), ());
        let mut state = AppData::new(&mut event_queue, target_display.to_string());
        // Roundtrip twice to sync the outputs
        for _ in 0..2 {
            event_queue
                .roundtrip(&mut state)
                .map_err(|e| format!("Failed to communicate with the wayland socket: {}", e))?;
        }

        state
            .output_manager
            .as_ref()
            .ok_or("Compositor does not support wlr_output_management_v1.")?;

        Ok(WaylandBackend { state, event_queue })
    }

    /// Receive (and send) all buffered messages across the wayland socket.
    fn read_socket(&mut self) {
        self.event_queue
            .roundtrip(&mut self.state)
            .expect("Failed to read display changes.");
    }

    /// Send all buffered messages across the wayland socket.
    /// Slightly cheaper than `read_socket`.
    fn write_socket(&self) {
        self.event_queue
            .flush()
            .expect("Failed to apply display changes.");
    }
}

impl DisplayManager for WaylandBackend {
    fn change_rotation_state(&mut self, new_state: &Orientation) {
        debug!("change_rotation_state socket read");
        self.read_socket();
        self.state.update_configuration(new_state.wayland_state);
        self.write_socket();
        debug!("change_rotation_state socket write");
    }

    fn get_rotation_state(&mut self) -> Result<Transform, String> {
        self.read_socket();
        self.state
            .current_transform
            .ok_or("Failed to get current display rotation".into())
    }
}

struct AppData {
    target_display_name: String,
    target_head: Option<ZwlrOutputHeadV1>,
    output_manager: Option<ZwlrOutputManagerV1>,
    current_config_serial: Option<u32>,
    current_transform: Option<Transform>,
    queue_handle: QueueHandle<AppData>,
}

/// Public interface

impl AppData {
    pub fn new(event_queue: &mut EventQueue<AppData>, target_display_name: String) -> Self {
        AppData {
            target_display_name,
            queue_handle: event_queue.handle(),
            target_head: None,
            output_manager: None,
            current_config_serial: None,
            current_transform: None,
        }
    }

    pub fn update_configuration(&mut self, new_transform: Transform) {
        if let (Some(output_manager), Some(serial), Some(head)) = (
            &self.output_manager,
            self.current_config_serial,
            &self.target_head,
        ) {
            let configuration = output_manager.create_configuration(serial, &self.queue_handle, ());
            let head_config = configuration.enable_head(head, &self.queue_handle, ());
            head_config.set_transform(new_transform);
            configuration.apply();
            debug!("configuraton applied");
        }
    }
}

/// Event handlers

impl Dispatch<wl_registry::WlRegistry, ()> for AppData {
    fn event(
        state: &mut Self,
        registry: &wl_registry::WlRegistry,
        event: wl_registry::Event,
        _: &(),
        _: &Connection,
        qh: &QueueHandle<AppData>,
    ) {
        if let wl_registry::Event::Global {
            name,
            interface,
            version,
        } = event
        {
            // println!("[{}] {} v{}", name, interface, version);
            if interface == "zwlr_output_manager_v1" {
                state.output_manager =
                    Some(registry.bind::<ZwlrOutputManagerV1, (), AppData>(name, version, qh, ()));
            }
        }
    }
}

impl Dispatch<ZwlrOutputManagerV1, ()> for AppData {
    fn event(
        state: &mut Self,
        _: &ZwlrOutputManagerV1,
        event: zwlr_output_manager_v1::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<AppData>,
    ) {
        if let zwlr_output_manager_v1::Event::Done { serial } = event {
            // println!("Current config: {}", serial);
            state.current_config_serial = Some(serial);
        }
    }

    event_created_child!(AppData, ZwlrOutputHeadV1, [
       zwlr_output_manager_v1::EVT_HEAD_OPCODE => (ZwlrOutputHeadV1, ()),
    ]);
}

impl Dispatch<ZwlrOutputHeadV1, ()> for AppData {
    fn event(
        state: &mut Self,
        head: &ZwlrOutputHeadV1,
        event: zwlr_output_head_v1::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<AppData>,
    ) {
        match event {
            zwlr_output_head_v1::Event::Name { name } => {
                if name == state.target_display_name {
                    // println!("Found target display: {}", name);
                    state.target_head = Some(head.clone());
                }
            }
            zwlr_output_head_v1::Event::Transform { transform } => {
                state.current_transform = Some(transform.into_result().unwrap())
            }
            _ => {}
        }
    }

    event_created_child!(AppData, ZwlrOutputModeV1, [
       zwlr_output_head_v1::EVT_CURRENT_MODE_OPCODE => (ZwlrOutputModeV1, ()),
       zwlr_output_head_v1::EVT_MODE_OPCODE => (ZwlrOutputModeV1, ()),
    ]);
}

impl Dispatch<ZwlrOutputModeV1, ()> for AppData {
    fn event(
        _state: &mut Self,
        _: &ZwlrOutputModeV1,
        _: zwlr_output_mode_v1::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<AppData>,
    ) {
    }
}

impl Dispatch<ZwlrOutputConfigurationV1, ()> for AppData {
    fn event(
        _state: &mut Self,
        config: &ZwlrOutputConfigurationV1,
        event: zwlr_output_configuration_v1::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<AppData>,
    ) {
        match event {
            zwlr_output_configuration_v1::Event::Succeeded => {
                // println!("Config applied successfully.");
                config.destroy();
            }
            zwlr_output_configuration_v1::Event::Failed => {
                // println!("Failed to apply new config.");
                config.destroy();
            }
            zwlr_output_configuration_v1::Event::Cancelled => {
                // println!("Config application cancelled.");
                config.destroy();
            }
            _ => {}
        }
    }
}

impl Dispatch<ZwlrOutputConfigurationHeadV1, ()> for AppData {
    fn event(
        _state: &mut Self,
        _: &ZwlrOutputConfigurationHeadV1,
        _: zwlr_output_configuration_head_v1::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<AppData>,
    ) {
    }
}
