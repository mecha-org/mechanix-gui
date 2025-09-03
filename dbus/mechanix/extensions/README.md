# Mechanix Extensions Service

A Rust service for managing and detecting hardware extensions in the Mechanix ecosystem through D-Bus.

## Overview

This service provides device detection and classification capabilities for the Mechanix system. It automatically detects hardware devices, classifies them by type and connection method, and exposes extension devices through a D-Bus interface.

## D-Bus Interface

The service exposes its functionality through the D-Bus interface:

- **Service**: `org.mechanix.Extensions`
- **Path**: `/org/mechanix/Extensions`
- **Interface**: `org.mechanix.Extensions`

## Usage

### Device Detection

```rust
use extensions::device::{Device, DeviceType, ConnectionType};

// Create a device from evdev
let device = Device::new(evdev_instance)?;

// Get device classification
let device_type = DeviceType::get_device_type(&evdev_instance);
let connection_type = ConnectionType::get_connection_type(&evdev_instance);
```

## Device Detection API Example
This recieve device event from all devices (only currently) and matches it with a specific keycode (Key_A in this case) and reports Event Added or Removed accordingly if its pressed or released.
```rust
use tokio;
use extensions::proxy::reciever::ExtensionServiceReceiver;
use extensions::events::ExtensionServiceEvent;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create receiver and get the event channel
    let (receiver, mut event_receiver) = ExtensionServiceReceiver::new().await?;

    // Start the receiver service
    tokio::spawn(async move {
        receiver.start_service().await.ok();
    });

    // Listen for device events
    while let Some(event) = event_receiver.recv().await {
        match event {
            ExtensionServiceEvent::Added(device) => {
                println!("Device added: {} ({})", device.name(), device.unique_id());
            }
            ExtensionServiceEvent::Removed(device) => {
                println!("Device removed: {}", device.name());
            }
        }
    }

    Ok(())
}
```

To run the service:

```bash
cargo run --example service
```

## Examples

To run the service:

```bash
cargo run --example service
```

To test device detection use the proxy example:

```bash
cargo run --example reciever
```
This will print detected devices and their classifications.

### Bevy Example
```rust
use bevy::prelude::*;
use extensions::plugin::{ ExtensionPlugin, DeviceInteruptEvent };
use extensions::device::Device;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ExtensionPlugin)
        .add_systems(Update, handle_extension_events)
        .run();
}

fn handle_extension_events(mut device_interupt_events: EventReader<DeviceInteruptEvent>) {
    // read events from DeviceInteruptEvent's event reader (later will be changed to only Extensions after detection)
        };
```
To test the Bevy example, which to demonstrate integration with the Bevy game engine, run:

```bash
cargo run --example bevy_example
```



### Extension Identification
The service automatically reads the `config.toml` file to identify which devices should be treated as extensions. Devices with unique IDs listed in the configuration will be classified as `DeviceType::Extension` and which KEY CODE represent the connection key, for matching and sending Connection Events via DBus.

#### Configuration
Extensions are configured through a `config.toml` file:

```toml
[[extensions]]
unique_id = "your-device-unique-id"
name = "Device Name"
connection_key = "KEY_B"

[[extensions]]
unique_id = "another-device-id"
name = "Another Device"
connection_key = "KEY_A"
```

## License

This project is licensed under the terms specified in the LICENSE