use crate::events::{AppEvents, NmEvents};
use futures::{FutureExt, SinkExt, StreamExt, channel::mpsc, select};
use networkmanager::{interfaces::wireless::NMState, service::NetworkManagerService};

pub async fn network_worker(mut tx: mpsc::Sender<AppEvents>, mut nm_rx: mpsc::Receiver<NmEvents>) {
    let network_manager = NetworkManagerService::new().await.unwrap();
    let mut enable_state_stream = network_manager
        .stream_wireless_enabled_status()
        .await
        .fuse();
    let mut device_state_stream = network_manager.stream_device_events().await.fuse();
    let mut strength_stream = network_manager
        .stream_active_network_strength()
        .await
        .fuse();

    loop {
        select! {
            event = nm_rx.next() => {
                if let Some(NmEvents::WirelessToggle { enabled }) = event {
                    let _ = network_manager.toggle_wireless(enabled).await;
                }
            },

            enable_state = enable_state_stream.next().fuse() => {
                if let Some(is_enabled) = enable_state {
                    let _ = tx.send(AppEvents::WirelessStatusChanged { enabled: is_enabled }).await;
                }
            },

            device_state = device_state_stream.next().fuse() => {
                if let Some(nm_state) = device_state {
                   match nm_state {
                        NMState::ConnectedGlobal => {
                            let _ = sync_connected_network(tx.clone()).await;
                        }
                        _ => {}
                    }
                }
            }

            strength_stream = strength_stream.next().fuse() => {
                if let Some(strength) = strength_stream {
                    let _ = tx.send(AppEvents::WirelessStrength { strength }).await;
                }
            }


        }
    }
}
async fn sync_connected_network(mut tx: mpsc::Sender<AppEvents>) {
    let network_manager = match NetworkManagerService::new().await {
        Ok(nm) => nm,
        Err(e) => {
            eprintln!("Failed to create NetworkManagerService: {}", e);
            return;
        }
    };

    match network_manager.list_networks().await {
        Ok(result) => {
            let connected_network = result.iter().find(|n| n.is_active).cloned();
            let _ = tx
                .send(AppEvents::ConnectedNetwork {
                    network: connected_network.clone(),
                })
                .await;
        }
        Err(e) => {
            eprintln!("Failed to get active network: {}", e);
            return;
        }
    };
}
