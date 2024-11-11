#[derive(Default, Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum WirelessStatus {
    On,
    #[default]
    Off,
    Connected(WirelessConnectedState),
    NotFound,
}

impl fmt::Display for WirelessStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            WirelessStatus::On => write!(f, "WirelessOn"),
            WirelessStatus::Off => write!(f, "WirelessOff"),
            WirelessStatus::Connected(state) => write!(f, "WirelessConnected({:?})", state),
            WirelessStatus::NotFound => write!(f, "WirelessNotFound"),
        }
    }
}