use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct RunCommands {
    pub notification: String,
}

impl Default for RunCommands {
    fn default() -> Self {
        Self {
            notification: String::new(),
        }
    }
}

#[derive(Debug, Deserialize, Clone, Serialize, Default)]
pub struct NotifierSettings {
    pub run_commands: RunCommands,
}
