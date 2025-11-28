use crate::events::AppEvents;
use futures::{FutureExt, SinkExt, StreamExt, channel::mpsc, select};
use networkmanager::service::NetworkManagerService;

pub async fn network_worker(mut tx: mpsc::Sender<AppEvents>) {
    let network_manager = NetworkManagerService::new().await.unwrap();
    let mut enable_state_stream = network_manager
        .stream_wireless_enabled_status()
        .await
        .fuse();
    let mut strength_stream = network_manager
        .stream_active_network_strength()
        .await
        .fuse();

    loop {
        select! {
            enable_state = enable_state_stream.next().fuse() => {
                if let Some(is_enabled) = enable_state {
                    let _ = tx.send(AppEvents::WirelessStatusChanged { enabled: is_enabled }).await;
                }
            },


            strength_stream = strength_stream.next().fuse() => {
                if let Some(strength) = strength_stream {
                    let _ = tx.send(AppEvents::WirelessStrength { strength }).await;
                }
            }


        }
    }
}
