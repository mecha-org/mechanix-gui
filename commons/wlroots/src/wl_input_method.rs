use wayland_client::{
    protocol::{
        wl_registry,
        wl_seat::{self, WlSeat},
    },
    Connection, Dispatch, EventQueue, QueueHandle,
};
use wayland_protocols_misc::zwp_input_method_v2::client::{
    zwp_input_method_manager_v2::{self, ZwpInputMethodManagerV2},
    zwp_input_method_v2::{self, ZwpInputMethodV2},
};

#[derive(Clone)]
pub struct InputFocusHandler {
    seat: Option<WlSeat>,
    input_manager: Option<ZwpInputMethodManagerV2>,
    pub input_method: Option<ZwpInputMethodV2>,
    input_active: bool,
    input_serial: u32,
}
impl InputFocusHandler {
    pub fn new(queue: &mut EventQueue<InputFocusHandler>) -> InputFocusHandler {
        let mut state = InputFocusHandler {
            seat: None,
            input_manager: None,
            input_method: None,
            input_active: false,
            input_serial: 0,
        };
        // need to roundtrip 3 times before we start getting deactivate/activate data
        queue.roundtrip(&mut state).unwrap();
        queue.roundtrip(&mut state).unwrap();
        queue.roundtrip(&mut state).unwrap();
        state
    }

    pub fn is_active(&self) -> bool {
        self.input_active
    }
}

impl Dispatch<wl_registry::WlRegistry, ()> for InputFocusHandler {
    fn event(
        state: &mut Self,
        registry: &wl_registry::WlRegistry,
        event: wl_registry::Event,
        _: &(),
        _: &Connection,
        qh: &QueueHandle<InputFocusHandler>,
    ) {
        if let wl_registry::Event::Global {
            name,
            interface,
            version,
        } = event
        {
            // println!("[{}] {} v{}", name, interface, version);
            if interface == "wl_seat" {
                registry.bind::<WlSeat, (), InputFocusHandler>(name, version, qh, ());
            }
            if interface == "zwp_input_method_manager_v2" {
                // println!("Creating input method");
                state.input_manager = Some(
                    registry.bind::<ZwpInputMethodManagerV2, (), InputFocusHandler>(
                        name,
                        version,
                        qh,
                        (),
                    ),
                );
            }
        }
    }
}

impl Dispatch<WlSeat, ()> for InputFocusHandler {
    fn event(
        state: &mut Self,
        seat: &WlSeat,
        event: wl_seat::Event,
        _: &(),
        _: &Connection,
        qh: &QueueHandle<InputFocusHandler>,
    ) {
        if let wl_seat::Event::Name { .. } = event {
            state.seat = Some(seat.to_owned());
            // eprintln!("Found seat: {}", name);
            if let Some(im) = state.input_manager.as_ref() {
                state.input_method = Some(im.get_input_method(seat, qh, ()));
            } else {
                eprintln!(
                    "Unable to bind input method management protocol, text input won't work."
                );
            }
        }
    }
}

impl Dispatch<ZwpInputMethodManagerV2, ()> for InputFocusHandler {
    fn event(
        _state: &mut Self,
        _: &ZwpInputMethodManagerV2,
        _: zwp_input_method_manager_v2::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<InputFocusHandler>,
    ) {
    }
}

impl Dispatch<ZwpInputMethodV2, ()> for InputFocusHandler {
    fn event(
        state: &mut Self,
        _: &ZwpInputMethodV2,
        event: zwp_input_method_v2::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<InputFocusHandler>,
    ) {
        // println!("Received input method event! {}", event.opcode());
        match event {
            zwp_input_method_v2::Event::Activate => {
                // eprintln!("Input method activated!");
                state.input_active = true;
            }
            zwp_input_method_v2::Event::Deactivate => {
                // eprintln!("Input method deactivated!");
                state.input_active = false;
            }
            zwp_input_method_v2::Event::Done => {
                // eprintln!("Received done event");
                state.input_serial += 1;
            }
            _ => {}
        }
    }
}
