mod proxies;

pub mod sound {
    use crate::proxies;
    pub use mechanix_desktop_dbus_server::{SinkInformationResponse, SourceInformationResponse};
    pub use proxies::sound_proxy::{NotificationStream, Sound};
}

pub mod power {
    use crate::proxies;
    pub use proxies::power_proxy::Power;
}
