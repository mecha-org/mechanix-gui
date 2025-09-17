use crate::events::ExtensionServiceEvent;
use crate::proxy::extensions::ExtensionServiceProxy;
use zbus::Connection;
use futures_lite::stream::StreamExt;
use tokio::sync::mpsc::{Receiver, Sender, channel};

pub struct ExtensionServiceReceiver {
    proxy: ExtensionServiceProxy<'static>,
    event_sender: Sender<ExtensionServiceEvent>,
}

impl ExtensionServiceReceiver {
    /// Create a new ExtensionServiceReceiver
    pub async fn new() -> Result<(Self, Receiver<ExtensionServiceEvent>), Box<dyn std::error::Error>> {
        // Create D-Bus connection
        let connection = Connection::session().await?;
        
        // Create proxy for the ExtensionService
        let proxy = ExtensionServiceProxy::new(&connection).await?;
        
        // Create channel for events
        let (event_sender, event_receiver) = channel(100);
        
        let receiver = Self {
            proxy,
            event_sender,
        };
        
        Ok((receiver, event_receiver))
    }
    
    /// Start the receiver service to listen for D-Bus signals and convert them to ExtensionServiceEvent
    pub async fn start_service(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Starting ExtensionService receiver...");

        // Create signal receivers
        let mut device_added_stream = self.proxy.receive_device_added().await?;
        let mut device_removed_stream = self.proxy.receive_device_removed().await?;

        println!("Listening for device events...");

        // Listen for signals and convert them to ExtensionServiceEvent
        loop {
            tokio::select! {
                msg = device_added_stream.next() => {
                    match msg {
                        Some(signal) => {
                            match signal.args() {
                                Ok(args) => {
                                    let device = args.device;
                                    // Send as ExtensionServiceEvent
                                    if let Err(e) = self.event_sender.send(ExtensionServiceEvent::Added(device)).await {
                                        eprintln!("Failed to send Added event: {}", e);
                                        break;
                                    }
                                }
                                Err(e) => {
                                    eprintln!("Error parsing device_added signal: {}", e);
                                }
                            }
                        }
                        None => {
                            println!("device_added stream ended");
                            break;
                        }
                    }
                }
                
                msg = device_removed_stream.next() => {
                    match msg {
                        Some(signal) => {
                            match signal.args() {
                                Ok(args) => {
                                    let device = args.device;
                                    println!("Received device removed signal: {}", device.name());
                                    
                                    // Send as ExtensionServiceEvent
                                    if let Err(e) = self.event_sender.send(ExtensionServiceEvent::Removed(device)).await {
                                        eprintln!("Failed to send Removed event: {}", e);
                                        break;
                                    }
                                }
                                Err(e) => {
                                    eprintln!("Error parsing device_removed signal: {}", e);
                                }
                            }
                        }
                        None => {
                            println!("device_removed stream ended");
                            break;
                        }
                    }
                }
            }
        }

        Ok(())
    }
} 