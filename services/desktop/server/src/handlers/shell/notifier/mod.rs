use anyhow::Result;
use command::spawn_command;
use std::time::Instant;
use tokio::sync::{mpsc, oneshot};

use crate::settings::home_button::HomeButtonSettings;

pub struct Notifier {
    configs: HomeButtonSettings,
}
impl Notifier {
    pub fn new(configs: HomeButtonSettings) -> Self {
        Self { configs }
    }

    pub async fn run(mut self) {}
}

async fn notify(run_command: String) -> Result<bool> {
    let _ = tokio::spawn(async move {
        if !run_command.is_empty() {
            let mut args: Vec<String> = vec!["-c".to_string()];
            args.push(run_command.clone());
            let res = spawn_command("sh".to_string(), args);
            println!("spawn_command res {:?}", res);
        }
    })
    .await;
    Ok(true)
}
