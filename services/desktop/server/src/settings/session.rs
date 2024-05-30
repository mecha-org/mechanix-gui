use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct RunCommands {
    pub lock_screen: String,
}

impl Default for RunCommands {
    fn default() -> Self {
        Self {
            lock_screen: String::new(),
        }
    }
}

#[derive(Debug, Deserialize, Clone, Serialize, Default)]
pub struct SessionSettings {
    pub run_commands: RunCommands,
}
