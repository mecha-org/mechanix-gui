mod proxies;

pub mod sound {
    use crate::proxies;
    pub use proxies::sound_proxy::{NotificationStream, Sound};
}

pub mod power {
    use crate::proxies;
    pub use proxies::power_proxy::Power;
}

pub mod security {
    use crate::proxies;
    pub use proxies::security_proxy::Security;
}
