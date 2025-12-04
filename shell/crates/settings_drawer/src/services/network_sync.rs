use crate::events::AppEvents;
use futures::{SinkExt, channel::mpsc};
use networkmanager::service::NetworkManagerService;

pub async fn sync_connected_network(
    mut tx: mpsc::Sender<AppEvents>,
    network_manager_service: &NetworkManagerService,
) {
    match network_manager_service.list_networks().await {
        Ok(result) => {
            let connected_network = result.iter().find(|n| n.is_active).cloned();
            let is_connected = result.iter().any(|n| n.is_active && !n.ssid.is_empty()); // IMP
           
            let _ = tx
                .send(AppEvents::ConnectedNetwork {
                    network: if is_connected {
                        connected_network.clone()
                    } else {
                        None
                    },
                })
                .await;
        }
        Err(e) => {
            eprintln!("Failed to get active network: {}", e);
            return;
        }
    };
}
