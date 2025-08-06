# Mechanix Configuration Client (mxconf-client)

A Rust client library for interacting with the Mechanix configuration service through D-Bus.

## Overview

This library provides a client interface to interact with the Mechanix configuration service (MxConf) using D-Bus. It
allows applications to:

- Get configuration settings
- Set configuration values
- List available schemas
- Watch for configuration changes
- List available keys within schemas
- Describe configuration keys

## Features

- Asynchronous D-Bus communication using zbus
- Error handling with anyhow
- JSON serialization/deserialization support
- Signal handling for configuration changes

## Usage

### Basic Usage

```rust
use mxconf_client;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Get a configuration setting
    let settings = mxconf_client::get_setting("your.key.here").await?;

    // Set a configuration value
    mxconf_client::set_setting("your.key.here", "value").await?;

    // List all schemas
    mxconf_client::list_schemas().await?;

    Ok(())
}
```

### Watching for Changes

```rust
use mxconf_client;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Watch for changes to a specific key
    mxconf_client::watch_setting("your.key.here").await?;

    Ok(())
}
```

## Dependencies

- `zbus` - For D-Bus communication
- `comfy-table` - For formatted table output
- `tokio` - For async runtime
- `serde_json` - For JSON serialization/deserialization
- `anyhow` - For error handling
- `futures-util` - For async utilities
- `log` - For logging

## Development

To run the examples:

```bash
cargo run --example client
```

## License

This project is licensed under the terms specified in the LICENSE file.
