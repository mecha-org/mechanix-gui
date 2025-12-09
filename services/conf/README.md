# 📚 MxConf (Mechanix Configuration Service)

MxConf is a Rust-based configuration service that acts as a database for storing configuration keys and their values.  
Configurations can be **Inserted, inspected, and modified** using the `mxconf` utility through **D-Bus** and **CLI interfaces**.

---

## 🛠 Features

- **File Watching**: Automatically detects new TOML configuration files in `/usr/share/mxconf/schemas`.
- **D-Bus Server**: Exposes configuration operations over D-Bus.
- **Command-Line Interface**: Provides commands to get, set, watch for changes, and list schemas.
- **Lockdown Mode**: Restrict specific settings from being changed by users.
- **Profiles**: Manage user-level and system-level defaults and overrides.
- **Embedded Database**: Stores validated data using [sled](https://github.com/spacejam/sled).
- **Namespaced Storage**: Configurations are grouped by namespace for efficient access.
- **Checksum Verification**: Avoids reprocessing unchanged configuration files.

---

## 🧩 Profiles, System Keyfiles, and User Keystore

### Profiles
- Installed with a default profile:  
  `/etc/mxconf/profile/default.toml`

Example:

```

[user]
keystore = "user"       \# User keystore stored in \$HOME/.conf/mxconf/

[system]
keyfiles = "system"     \# System keyfiles in /etc/mxconf/keyfiles/

```

- The active profile is determined by the `MXCONF_PROFILE` environment variable.
- If unset, defaults to `default`.

---

### System Keyfiles
- Location: `/etc/mxconf/keyfiles/<name>`
- Default: `/etc/mxconf/keyfiles/system`

Applications can install their own system keyfiles using naming convention:

```

XX-org.<namespace>.<app_name>.toml

```

Example: `01-org.mechanix.launcher.toml`

#### Example Content: `00-org.mechanix.keyboard.toml`

```

[general]
enabled = { value = "true", locked = "true" }

[theme]
mode = "dark"

```

✅ Both **inline tables** (`{}`) and **direct assignments** (`key = value`) are valid in TOML.

---

## 🔧 Configuration Schema

- One schema per **namespace** (e.g., `org.mechanix.launcher`)
- Written in **TOML**
- To Register a new schema, place a new TOML file in `/usr/share/mxconf/schemas`.

### Example Schema: `org.mechanix.launcher`

```

[appearance]
theme = { type = "string", key = "color", default = "black" }
borderRadius = { type = "number", key = "radius", default = 8 }
darkMode = { type = "bool", key = "enabled", default = false }

```

---

## ⚙️ Installation

### Prerequisites
- Rust (2021 edition or later)
- Cargo

### Build from Source

```shell

$ https://github.com/mecha-org/mechanix-gui.git -b pre-release
$ cd services/conf
$ cargo build --release

```

Run the server:

```shell

$ cargo run --release

```

Enable debug logging:

```shell

$ RUST_LOG=none,mxconf=debug ../target/release/mxconf -s

```

---

## 🚀 Usage

### Server Mode

1. Start the server:

```shell

$ ./mxconf -s

```

2. Place your schema files in `/usr/share/mxconf/schemas`.

The server will automatically:
- Detect new TOML files
- Validate them

3. Configurations are stored in:

```shell

$ ~/.config/mxconf/db

```

---

### CLI Mode

Interact with the running server from the command line:

- **Get** a setting:

```

./mxconf get <key>
./mxconf get <key_expr>   \# supports wildcards

```

- **Set** a setting:

```shell

$ ./mxconf set <key> <value>

```

- **Watch** for changes:

```shell

$ ./mxconf watch <key>
$ ./mxconf watch <key_expr>

```

- **List available schemas**:

```shell

$ ./mxconf list-schemas

```

---
### D-Bus Interface

MXCONF exposes its configuration operations via **D-Bus**.

- **Bus name:** `org.mechanix.MxConf`
- **Object path:** `/org/mechanix/MxConf`
- **Interface:** `org.mechanix.MxConf`
---

## 🚀 Methods

### DescribeKey
Describe schema and its key’s metadata.

busctl call org.mechanix.MxConf /org/mechanix/MxConf org.mechanix.MxConf DescribeKey ss "<schema>" "<key>"

Example:
```shell
$ busctl call org.mechanix.MxConf /org/mechanix/MxConf org.mechanix.MxConf DescribeKey ss "org.mechanix.launcher" "theme"
```

### GetSetting

Return the current value for a key.

busctl call org.mechanix.MxConf /org/mechanix/MxConf org.mechanix.MxConf GetSetting s "<key>"

Example:

```shell
$ busctl call org.mechanix.MxConf /org/mechanix/MxConf org.mechanix.MxConf GetSetting s "org.mechanix.launcher.theme"
```


### ListKeys

List all keys for a given schema.

busctl call org.mechanix.MxConf /org/mechanix/MxConf org.mechanix.MxConf ListKeys s "<schema>"

Example:
```shell
$ busctl call org.mechanix.MxConf /org/mechanix/MxConf org.mechanix.MxConf ListKeys s "org.mechanix.launcher.theme"
```

### ListSchemas
List all configuration schemas.

busctl call org.mechanix.MxConf /org/mechanix/MxConf org.mechanix.MxConf ListSchemas

Example:
```shell
$ busctl call org.mechanix.MxConf /org/mechanix/MxConf org.mechanix.MxConf ListSchemas
```

### SetSetting

Set the value for a key.

busctl call org.mechanix.MxConf /org/mechanix/MxConf org.mechanix.MxConf SetSetting ss "<key>" "<value>"

Example:
```shell
$ busctl call org.mechanix.MxConf /org/mechanix/MxConf org.mechanix.MxConf SetSetting ss "org.mechanix.launcher.theme" "dark"
```

> **Note:** The server **must be running** with `-s` for CLI commands to work.**
