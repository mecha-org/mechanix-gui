use zbus::{ interface, Connection, object_server::SignalEmitter };
use tokio::{ sync::mpsc::{ Receiver, Sender, channel } };
use crate::errors::{ Result, Error };
use crate::events::ExtensionServiceEvent;
use crate::detection::service::watch_hotplug;
use zvariant::ObjectPath;
use crate::device::Device;

// this service will add recieve and send all HotPlug Events (Added, Removed) --> org.mechanix.Extensions --> recieved by ExtensionServiceProxy.
#[derive(Debug, Clone)]
pub struct ExtensionService {
    sender: Option<Sender<ExtensionServiceEvent>>,
}

impl ExtensionService {
    fn new() -> (Self, Receiver<ExtensionServiceEvent>) {
        let (sender, reciever) = channel(100);
        let service = Self {
            sender: Some(sender),
        };
        (service, reciever)
    }

    // (connection_dbus_extension, ExtensionService{sender}, Reciever<ExtensionServiceEvent>) => detection/service.rs has a will use this sender to send (Device) as a event
    pub async fn create_connection() -> Result<
        (Connection, ExtensionService, Receiver<ExtensionServiceEvent>)
    > {
        let connection = Connection::session().await.map_err(|e|
            Error::DbusString(format!("Failed to create D-Bus connection: {}", e))
        )?;

        let (extension_service, receiver) = ExtensionService::new();
        connection
            .object_server()
            .at("/org/mechanix/Extensions", extension_service.clone()).await
            .map_err(|e| Error::DbusString(format!("Failed to register object: {}", e)))?;
        connection
            .request_name("org.mechanix.Extensions").await
            .map_err(|e| Error::DbusString(format!("Failed to request D-Bus name: {}", e)))?;
        Ok((connection, extension_service, receiver))
    }

    // sub-part of start_service() : will be called in start_service to send Signals via Dbus when reciever recieves something.
    async fn handle_events(
        mut receiver: Receiver<ExtensionServiceEvent>,
        signal_emitter: SignalEmitter<'static>
    ) {
        while let Some(event) = receiver.recv().await {
            match event {
                ExtensionServiceEvent::Added(device) => {
                    println!("device added event: {:?}", device);
                    if let Err(e) = Self::device_added(&signal_emitter,device).await {
                        eprintln!("Failed to emit device_added signal: {}", e);
                    }
                }
                ExtensionServiceEvent::Removed(device) => {
                    println!("device removed event: {:?}", device);
                    if let Err(e) = Self::device_removed(&signal_emitter,device).await {
                        eprintln!("Failed to emit device_removed signal: {}", e);
                    }
                }
            }
        }
    }

    pub async fn start_service() -> Result<()> {
        println!("Starting extension service...");

        // Create D-Bus connection and service
        let (connection, extension_service, receiver) = match
            ExtensionService::create_connection().await
        {
            Ok(result) => {
                println!("D-Bus connection established successfully");
                result
            }
            Err(e) => {
                eprintln!("Failed to create D-Bus connection: {}", e);
                return Err(e);
            }
        };

        // Get the sender from the extension service
        let sender = match extension_service.sender.clone() {
            Some(sender) => sender,
            None => {
                let error = Error::DbusString("Extension service sender not available".to_string());
                eprintln!("Error: {}", error);
                return Err(error);
            }
        };

        // Get signal emitter for the object using the new pattern
        let signal_emitter = SignalEmitter::from_parts(
            connection.clone(),
            ObjectPath::try_from("/org/mechanix/Extensions").map_err(|e|
                Error::DbusString(format!("Invalid object path: {}", e))
            )?
        );

        println!("Starting hotplug monitoring and event handling...");

        // Spawn the hotplug watcher task
        let hotplug_sender = sender.clone();
        let hotplug_task = tokio::spawn(async move {
            if let Err(e) = watch_hotplug(hotplug_sender).await {
                eprintln!("Hotplug monitoring failed: {}", e);
            }
        });

        // Spawn the event handler task
        let event_handler_task = tokio::spawn(async move {
            Self::handle_events(receiver, signal_emitter).await;
        });

        println!("Extension service is running...");

        // Wait for either task to complete (or fail)
        tokio::select! {
            result = hotplug_task => {
                match result {
                    Ok(_) => println!("Hotplug monitoring task completed"),
                    Err(e) => eprintln!("Hotplug monitoring task failed: {}", e),
                }
            }
            result = event_handler_task => {
                match result {
                    Ok(_) => println!("Event handler task completed"),
                    Err(e) => eprintln!("Event handler task failed: {}", e),
                }
            }
        }

        println!("Extension service stopped");
        std::future::pending::<()>().await;

        Ok(())
    }
}

#[interface(name = "org.mechanix.Extensions")]
impl ExtensionService {
    #[zbus(signal)]
    async fn device_added(signal_ctxt: &SignalEmitter<'_>, device: Device) -> zbus::Result<()>;

    #[zbus(signal)]
    async fn device_removed(signal_ctxt: &SignalEmitter<'_>, device: Device) -> zbus::Result<()>;
}
