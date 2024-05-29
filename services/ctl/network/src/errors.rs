//make a sturct for WirelessNetworkError and implement the Error trait for it as we did for all the other errors try to includ all the possible error code that you can think of while working with linux and wifi

#[derive(Debug, Default, Clone, Copy)]
pub enum WirelessNetworkErrorCodes {
    #[default]
    NoWirelessNetworkFound,
    UnableToTurnOnWirelessNetwork,
    UnableToTurnOffWirelessNetwork,
    UnableToConnectToWirelessNetwork,
    UnableToDisconnectWirelessNetwork,
    UnableToGetWirelessNetworkStatus,
    UnableToRemoveWirelessNetwork,
    Unknown,
    WrongPsk,
    PendingSelect,
    NotFound,
    InvalidNetworkId,
    Timeout,
    AlreadyConnected,
}

impl std::fmt::Display for WirelessNetworkErrorCodes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            WirelessNetworkErrorCodes::NoWirelessNetworkFound => {
                write!(f, "NoWirelessNetworkFound")
            }
            WirelessNetworkErrorCodes::UnableToTurnOnWirelessNetwork => {
                write!(f, "UnableToTurnOnWirelessNetwork")
            }
            WirelessNetworkErrorCodes::UnableToTurnOffWirelessNetwork => {
                write!(f, "UnableToTurnOffWirelessNetwork")
            }
            WirelessNetworkErrorCodes::UnableToConnectToWirelessNetwork => {
                write!(f, "UnableToConnectToWirelessNetwork")
            }
            WirelessNetworkErrorCodes::UnableToDisconnectWirelessNetwork => {
                write!(f, "UnableToDisconnectWirelessNetwork")
            }
            WirelessNetworkErrorCodes::UnableToGetWirelessNetworkStatus => {
                write!(f, "UnableToGetWirelessNetworkStatus")
            }
            WirelessNetworkErrorCodes::UnableToRemoveWirelessNetwork => {
                write!(f, "UnableToRemoveWirelessNetwork")
            }
            WirelessNetworkErrorCodes::WrongPsk => write!(f, "WrongPsk"),
            WirelessNetworkErrorCodes::PendingSelect => write!(f, "PendingSelect"),
            WirelessNetworkErrorCodes::NotFound => write!(f, "NotFound"),
            WirelessNetworkErrorCodes::InvalidNetworkId => write!(f, "InvalidNetworkId"),
            WirelessNetworkErrorCodes::Timeout => write!(f, "Timeout"),
            WirelessNetworkErrorCodes::AlreadyConnected => write!(f, "AlreadyConnected"),
            WirelessNetworkErrorCodes::Unknown => write!(f, "Unknown"),
        }
    }
}

#[derive(Debug)]
pub struct WirelessNetworkError {
    pub code: WirelessNetworkErrorCodes,
    pub message: String,
}

impl std::fmt::Display for WirelessNetworkError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "(code: {:?}, message: {})", self.code, self.message)
    }
}

impl WirelessNetworkError {
    pub fn new(code: WirelessNetworkErrorCodes, message: String) -> Self {
        WirelessNetworkError { code, message }
    }
}
