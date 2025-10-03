# MxSearch

*A simple and fast search engine for your apps and files.*


## Overview

This service indexes configuration schemas into a Tantivy search index.
Each config file is parsed and indexed with its metadata and nested action sections.
It maintains data consistency by using checksum validation to avoid unnecessary re-indexing.

- Applications installed on the system register their **App Actions** to **mxsearch**.
- This allows mxsearch to return App Actions as part of search results.
- App Actions are stored as **TOML** files inside the directory:  
  `/usr/share/mxsearch/actions`

## ✨ Features

### 🖥️ Apps Search

MxSearch supports searching installed Linux applications using `.desktop` files.

- **Configurable App Search Service**
    - Easily enable/disable app search.
    - Define custom paths to your `.desktop` files via the config file.
    - Define a set of fields to index.

- **Crate: `apps`**
    - Create a new app search service instance.
    - Watch `.desktop` files and debounce index updates.
    - On remove of `.desktop` files, remove the app from the index.
    - Allow free-form searching of indexed applications.
    - Configure searchable fields in a settings file.
    - Graceful shutdown support (e.g. optional task cancellation).
    - Load existing entries from the applications directory.
        - Check if the path matches in the index. if not then continue.
        - If the path matches in index, then validate checksum.
        - If checksum matches, then continue.
        - If checksum does not match, then delete the existing entry from the index and add a new entry.
        - Refresh the index with a new entry.

---

### 📂 Files Search

MxSearch can index and query files from a given directory.

- **Crate: `files`**
    - Configure a dir to watch, and index files.
    - Allow free-form searching of indexed files.
    - Configure searchable fields in a settings file.
        - searchable_fields = [
          "file_type",
          "name",
          "content"
          ]
    - Load existing entries from the provided directory.
    - We can configure the depth of the directory to be indexed.
    - We can configure the allowed extensions to be indexed.
    - We can configure the max size of the file content to be indexed.

### ⚙️ App Actions Search
MXSEARCH supports indexing **App Actions** to return actionable results in searches.

Applications register their actions via TOML files stored at: `/usr/share/mxsearch/actions/`

Each app registers a TOML file named: `org.mechanix.<AppName>.toml`

Example for the Settings app:
`/usr/share/mxsearch/actions/org.mechanix.Settings.toml`

App Action Toml Format:
```toml
Name = "Settings"
Icon = "settings-icon"
Exec = "/usr/bin/settings-app"

[EnableWifi]
Action = "Enable WiFi"
Description = "Enable wireless network"
Arg = { path = "network" }

[EnableBluetooth]
Action = "Enable Bluetooth"
Description = "Enable bluetooth"
Arg = { path = "bluetooth" }

[Files]
Action = "Search Files"
Description = "Search by file name"
Arg = { path = "%KEYWORD%" }
```


---

### Indexing and Refresh

- MXSEARCH indexes the `/usr/share/mxsearch/actions` directory.
- Each TOML file is checksummed.
- When a file changes or the service starts, the actions are refreshed.
- The `%KEYWORD%` placeholder is dynamically replaced by the user’s search query when invoking actions.

---



## Example: Settings App Actions file

```toml
name = "Settings"
icon = "path/to/icon.png"
description = "Manage system settings"
exec = "mechanix-settings"

[EnableWifi]
action = "Enable WiFi"
description = "Enable wireless network"
arg = { path = "network" }

[Files]
Action = "Search Files"
Description = "Search by file name"
Arg = { path = "%KEYWORD%" }
```

File path: `/usr/share/mxsearch/actions/org.mechanix.Settings.toml`

- `%KEYWORD%` is a reserved placeholder that passes the user's search key to the app action.

## Functionality

- mxsearch indexes the `/usr/share/mxsearch/actions` directory.
- It uses checksums to detect changes and refresh the index accordingly.
- Endpoints include:
    - Search App Actions

---

## Workflow

### On Service Start

1. Compute the checksum of the config file.
2. Search the Tantivy index for documents matching the file path.
3. Compare stored checksum(s) with the newly computed checksum.
4. If checksum matches, no action needed (index is current).
5. If not, parse the config file and re-index all action documents with the updated checksum.

### 🛠️ Configuration Example (`settings.toml`)



```toml
[general]
[apps]
enable_search_apps = false
desktop_apps_dir = "/usr/share/applications"
index_dir = ".config/mxsearch/index/applications"
search_limit = 1000
searchable_fields = [
    "type",
    "name",
    "generic_name",
    "comment",
    "keywords",
    "categories",
    "path",
    "checksum"
]
[files]
enable_search_files = true
files_dir_to_watch = "/home"
index_dir = ".config/mxsearch/index/files"
max_depth = 5
search_limit = 1000
read_file_content_upto_in_kb = 100
searchable_fields = [
    "file_type",
    "name",
    "content"
]
allowed_extensions = ["txt", "yaml", "rtf", "xml", "toml"]
[app_actions]
enable_search = true
index_dir = ".config/mxsearch/index/app_actions"
schema_dir = "/usr/share/mxsearch/actions"
search_limit = 15
searchable_fields = [
    "action",
]
```


## ⚙️ Installation

### Prerequisites
- Rust (2021 edition or later)
- Cargo

### Build from Source

```

https://github.com/mecha-org/mechanix-gui.git -b pre-release
cd services/search

cargo build --release

```

Run the server:

```

cargo run --release

```

Enable debug logging:

```

RUST_LOG=none,mxconf=debug ../target/release/mxconf -s

```

## 🚀 Running MxSearch

```bash
 RUST_LOG=none,search=debug,apps=debug,files=debug cargo run
```

## TODOs

Apps

- [ ] Allow index applications from a custom directory ex: snap packages, flatpak packages.
- [ ] While parsing desktop entry, getting single value from multiple value fields. ex: categories, keywords