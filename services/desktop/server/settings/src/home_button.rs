use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct RunCommands {
    pub app_switcher: String,
}

impl Default for RunCommands {
    fn default() -> Self {
        Self {
            app_switcher: String::new(),
        }
    }
}

#[derive(Debug, Deserialize, Clone, Serialize, Default)]
pub struct HomeButtonSettings {
    pub run_commands: RunCommands,
    pub min_time_long_press: u64,
}
