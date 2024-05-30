mod sound_interface;
pub use sound_interface::SoundBusInterface;

pub use sound_interface::sound_event_notification_stream;

mod power_interface;
pub use power_interface::PowerBusInterface;

mod security_interface;
pub use security_interface::SecurityBusInterface;
