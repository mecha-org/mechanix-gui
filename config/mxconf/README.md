# MXCONF (Mechanix Configuration Service)

A Rust-based configuration server that monitors schema files, validates them, and manages them efficiently in an
embedded database with D-Bus and CLI interfaces.

## Features

- **File Watching**: Automatically detects new TOML configuration files in the `/usr/share/mxconf/schemas` directory.
- **D-Bus Server**: Exposes a D-Bus interface for retrieving and managing configurations.
- **Command-Line Interface**: Provides CLI commands to get, set, watch for configuration changes, and list available
  schemas.
- **Lock down specific settings**: Use the lockdown mode in mxconf to prevent users from changing specific settings.
- **Manage user and system settings**: Use the profile to manage
- **Schema Validation**: Validates TOML files.
- **Embedded Database**: Stores validated configurations in a local embedded database
  using [sled](https://github.com/spacejam/sled).
- **Namespaced Storage**: Organizes configurations by namespace for efficient retrieval.
- **Checksum Verification**: Prevents duplicate processing of unchanged files.
- **Robust Error Handling**: Comprehensive error handling throughout the codebase.
- **Well-Documented Code**: Clear documentation for all functions and modules.

## Project Structure

The `/src` folder contains the main logic, organized as follows:

- `main.rs`: Entry point; parses CLI arguments, starts server or CLI mode.
- `cli_client.rs`: Handles CLI commands, including getting, setting, watching for changes, and listing schemas.
- `database.rs`: Handles embedded database (sled) operations, including namespaced storage.
- `server.rs`: Exposes configuration operations over D-Bus.
- `error.rs`: Defines custom error types for robust error handling.
- `utils.rs`: Utility functions used across modules.

## Installation

### Prerequisites

- Rust and Cargo (2021 edition or later)

### Building from Source

1. Clone the repository:
   ```
   git clone https://github.com/mecha-org/mxconf.git
   cd mxconf
   ```

2. Build the project:
   ```
   cargo build --release
   ```

3. Run the server:
   ```
   cargo run --release
   ```
   Debug logging:
   ```
    RUST_LOG=none,mxconf=debug ./target/release/mxconf -s
   ```

## Usage

### Server Mode

1. Start the server:
   ```
   cargo run --release -- -s
   ```
   or
   ```
   ./mxconf -s
   ```

2. Place TOML configuration files in the `schemas` directory. The server will automatically:
    - Detect a new TOML file
    - Validate them

3. The server stores configurations in `~/.config/mxconf/db` using the sled embedded database.

### CLI Mode

The application also provides a command-line interface for interacting with the configuration server:

1. Get a setting value (wildcard supported):
   ```
   cargo run --release -- get <key/key_expr>
   ```
   or
   ```
   ./mxconf get <key>
   ```
2. Set a setting value:
   ```
   cargo run --release -- set <key> <value>
   ```
   or
   ```
   ./mxconf set <key> <value>
   ```

3. Watch for changes to a setting:
   ```
   cargo run --release -- watch <key>
   ```
   or
   ```
   ./mxconf watch <key/key_expr>
   ```

4. List all available schemas:
   ```
   cargo run --release -- list-schemas
   ```
   or
   ```
   ./mxconf list-schemas
   ```

> **Note:** The server must be running (using the `-s` option) for the CLI commands to work.

