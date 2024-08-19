use mctk_core::reexports::smithay_client_toolkit::reexports::calloop::channel::Sender;
use std::time::Duration;

use crate::AppMessage;
pub struct NetworkingHandle {
    app_channel: Sender<AppMessage>,
}

impl NetworkingHandle {
    pub fn new(app_channel: Sender<AppMessage>) -> Self {
        NetworkingHandle { app_channel }
    }

    pub async fn run(&self) {
        loop {
            let is_online = online::check(None).is_ok();
            let _ = &self.app_channel.send(AppMessage::Net { online: is_online });
            tokio::time::sleep(Duration::from_secs(5)).await;
        }
    }
}
