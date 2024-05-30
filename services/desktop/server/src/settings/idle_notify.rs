use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct Periods {
    pub display_off: f32,
    pub screen_lock: f32,
}

impl Default for Periods {
    fn default() -> Self {
        Self {
            display_off: 10.,
            screen_lock: 30.,
        }
    }
}

#[derive(Debug, Deserialize, Clone, Serialize, Default)]
pub struct IdleNotifySettings {
    pub periods: Periods,
}
