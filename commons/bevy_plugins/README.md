# NetworkManagerPlugin

A Bevy plugin that provides network management functionality through DBus integration with NetworkManager.

## Features

The plugin handles the following network-related actions:

### WiFi Control
- **Toggle WiFi**: Enable or disable the WiFi adapter
- **List Networks**: Retrieve a list of available wireless networks
- **Connect to Network**: Connect to a wireless network with optional password
- **Connect to Saved Network**: Connect to a previously saved network
- **Forget Saved Network**: Remove a saved network from NetworkManager
- **Disconnect Network**: Disconnect from the current network

### Event Subscriptions
- **Device Events**: Subscribe to network device state changes
- **Access Points Events**: Subscribe to wireless access points updates

### Plugin Setup
```rust
use bevy::prelude::*;
fn main() {
    App::new().add_plugins(NetworkManagerServicePlugin);
    // ... other plugins and setup .run(); }

```

### Usage
```rust
fn do_network_action(mut event_writer: EventWriter<NetworkActionEvent>) {
    event_writer.write(NetworkActionEvent(NetworkAction::ToggleWifi(true)));
}
fn wait_action_result(mut event_reader: EventReader<NetworkResultEvent>) {
    for event in event_reader.iter() {
        println!("Network action result: {}", event.0);
    }
}
```