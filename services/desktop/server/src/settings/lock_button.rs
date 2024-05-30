use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct RunCommands {
    pub power_options: String,
}

impl Default for RunCommands {
    fn default() -> Self {
        Self {
            power_options: String::new(),
        }
    }
}

#[derive(Debug, Deserialize, Clone, Serialize, Default)]
pub struct LockButtonSettings {
    pub run_commands: RunCommands,
    pub min_time_long_press: u64,
}
