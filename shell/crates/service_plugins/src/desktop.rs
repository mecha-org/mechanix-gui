use bevy::log::{error, info};
use bevy::prelude::*;
use bevy::prelude::{Event, Resource};
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use system_dbus::hw_button_client::{HwButton, NotificationStream};
use system_dbus::service::DesktopService;
use system_dbus::KeyEvent;

#[derive(Resource, Default)]
pub struct PowerButtonEvent(pub KeyEvent);

#[derive(Default)]
pub struct PowerButtonInterruptReceiver(pub Option<mpsc::Receiver<KeyEvent>>);

#[derive(Event)]
pub struct HwActionEvent(pub HwAction);

#[derive(Debug, Clone)]
pub enum HwAction {
    StreamPowerButtonInterrupt,
}

#[derive(Resource, Default)]
pub struct ServiceState {
    pub initialized: bool,
    pub stream_started: bool,
}

#[derive(Debug, Clone)]
pub enum ErrorType {
    ActionFailed { action: HwAction, message: String },
}

pub struct DesktopPlugin;

impl Plugin for DesktopPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .insert_resource(ServiceState::default())
            .insert_non_send_resource(PowerButtonInterruptReceiver::default())
            .insert_resource(PowerButtonEvent::default())
            .add_event::<HwActionEvent>()
            .add_systems(
                Update,
                (start_initial_streams_if_service_ready, handle_desktop_event),
            )
            .add_systems(Update, poll_power_button_interrupt);
    }
}
fn setup() {
    info!("starting up");
}
fn poll_power_button_interrupt(
    interrupt_receiver: NonSendMut<PowerButtonInterruptReceiver>,
    mut power_button_event: ResMut<PowerButtonEvent>,
) {
    if let Some(power_status_receiver) = (&interrupt_receiver.0) {
        if let Ok(key_event) = power_status_receiver.try_recv() {
            info!("hw power button key event received: {:?} ", key_event);
            power_button_event.0 = key_event;
        }
    }
}
fn start_initial_streams_if_service_ready(
    mut state: ResMut<ServiceState>,
    mut events: EventWriter<HwActionEvent>,
) {
    // Only start once, and only when the service is initialized
    if !state.stream_started {
        events.write(HwActionEvent(HwAction::StreamPowerButtonInterrupt));
        state.stream_started = true;
    }
}

fn handle_desktop_event(
    mut events: EventReader<HwActionEvent>,
    mut power_button_interrupt_receiver: NonSendMut<PowerButtonInterruptReceiver>,
) {
    for event in events.read() {
        let HwActionEvent(action) = event;
        match action {
            HwAction::StreamPowerButtonInterrupt => {
                let desktop_service = DesktopService::new();
                info!("desktop action: stream power button");
                let receiver =
                    pollster::block_on(desktop_service.stream_hw_button_interrupt_event());
                power_button_interrupt_receiver.0 = Some(receiver);
            }
        } // Add more as needed
    }
}
