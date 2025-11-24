# MxSearch

*A simple and fast search engine for your apps and files.*

## Overview

This service indexes configuration schemas into a [Tantivy](https://docs.rs/tantivy/latest/tantivy/) search index.
Each config file is parsed and indexed with its metadata and nested action sections.
It maintains data consistency by using checksum validation to avoid unnecessary re-indexing.

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

MxSearch supports indexing **App Actions** to return actionable results in searches.

- Applications installed on the system register their **App Actions** to **mxsearch**.
- This allows mxsearch to return App Actions as part of search results.
- App Actions are stored as **TOML** files inside the directory:  `/usr/share/mxsearch/actions`
- Each TOML file is checksummed.
- The `%KEYWORD%` placeholder is dynamically replaced by the user’s search query when invoking actions.
  Each app registers a TOML file named: `org.mechanix.<AppName>.toml`

Example for the Settings app: `/usr/share/mxsearch/actions/org.mechanix.Settings.toml`

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

#### Example: Settings App Actions file

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

### ⚙️ Sources Search Service

#### What it is
The Sources Search Service lets you ingest and search your own items (notes, music, web links, etc.) alongside system results. It maintains a Tantivy index, exposes a D‑Bus API for upserts/deletes/queries, and automatically removes indexed items when their associated application entry is uninstalled.

#### Key features
- Single-writer Indexer actor for safe, fast writes (batched and periodically committed).
- Async ingestion via D‑Bus: upsert and delete by `unique_id`.
- File-system watcher on `.desktop` entries: when a desktop file is removed, corresponding items are deleted from the index using `source_entry_path`.
- Query across multiple fields with configurable `searchable_fields` and `search_limit`.

#### D‑Bus interface
- Bus name: `org.mechanix.MxSearch`
- Object path: `/org/mechanix/MxSearch`
- Interface: `org.mechanix.MxSearch`

Methods you can call from clients:
- `SearchExternal(search: &str) -> Vec<ExternalSearchResult>`
- `UpsertMetadata(metadata: Vec<UpsertMetadata>) -> bool`
- `DeleteMetadataByIds(ids: Vec<String>) -> bool`

#### Data model
- Upserting requires a list of `UpsertMetadata` objects:
    - `source: String` — logical source, e.g., `notes`, `music`.
    - `unique_id: String` — your stable identifier for the item.
    - `uri: String` — e.g., `file:///...` or an HTTP URL.
    - `title: String`
    - `subtitle: String`
    - `description: String`
    - `keywords: Vec<String>`
    - `icon: String` — optional icon name or URI.
    - `thumbnail: String` — optional thumbnail URI/path.
    - `last_modified: u64` — unix timestamp (e.g., seconds or millis; consistent across your system).
    - `content: Option<String>` — full-text (optional, can be large).
    - `source_entry_path: String` — the backing `.desktop` file path this item is associated with. If that file is removed, the item will be auto-removed from the index.

- Search returns a list of `SourcesSearchResult`:
    - `source, uri, title, icon, thumbnail, last_modified, content, unique_id, score`

Note: Fields that are returned depend on what is marked `STORED` in the schema.

#### Configuration (settings.toml)
```toml
# External (notes, music, etc.)
[sources]
# Enable the ExternalService (ingestion/search for sources items)
enable_search = true

# Index location (relative to $HOME)
index_dir = ".config/mxsearch/index/external"

# Directory of desktop entries to watch; recursive
app_dir = "/usr/share/applications"

# Tantivy writer heap (bytes)
target_memory_usage_in_bytes = 50_000_000

# Search behavior
search_limit = 25
searchable_fields = [
  "uri", "title", "subtitle", "keywords", "description", "content"
]
```

#### How it works (internals at a glance)
- On startup, the service builds the index schema and spawns a dedicated `Indexer` task that owns the Tantivy `IndexWriter`.
- The server keeps an `IndexReader` for queries and a channel to the Indexer for write commands:
    - `Upsert(Vec<UpsertMetadata>)`
    - `RemoveByUniqueIds(Vec<String>, need_commit: bool)`
    - `RemoveByPath(String)` — used by the watcher when a desktop file is removed.
    - `Flush`, `Shutdown`
- The Indexer batches operations and commits periodically (and on demand) so new data becomes searchable shortly after ingestion.
- A file watcher observes the configured `app_dir` recursively; on remove events it sends `RemoveByPath(source_entry_path)` to delete matching indexed documents.

#### Quick start (client perspective)
- Upsert some items via D‑Bus (pseudo-Rust with `zbus`):
```rust
let proxy = zbus::ProxyBuilder::new(&conn)
    .destination("org.mechanix.MxSearch")?
    .interface("org.mechanix.MxSearch")?
    .path("/org/mechanix/MxSearch")?
    .build()
    .await?;

let items = vec![UpsertMetadata {
    source: "notes".into(),
    unique_id: "note-123".into(),
    uri: "file:///home/user/notes/123.md".into(),
    title: "How to win".into(),
    subtitle: "Win over obstacles".into(),
    description: "My first note".into(),
    keywords: vec!["howto".into(), "motivation".into()],
    icon: "text-x-markdown".into(),
    thumbnail: "".into(),
    last_modified: 1_763_720_442,
    content: Some("This is something very interesting".into()),
    source_entry_path: "/usr/share/applications/Alacritty.desktop".into(),
}];

let ok: bool = proxy.call("UpsertMetadata", &(items)).await?;
```

- Search:
```rust
let results: Vec<ExternalSearchResult> = proxy
    .call("SearchExternal", &("how to"))
    .await?;
```

- Delete by IDs:
```rust
let ok: bool = proxy
    .call("DeleteMetadataByIds", &(vec!["note-123".to_string()]))
    .await?;
```

#### Operational notes
- Commit latency: the Indexer commits periodically; results may appear with a small delay. You can add a `Flush` command if you require immediate visibility after critical upserts.
- Watcher deletes: For automatic deletion to work, keep `source_entry_path` consistent with the actual `.desktop` file used by your item’s source application.
- Sorting: If you need ordering by recency, ensure `last_modified` is indexed as a fast field and sort at query time (future enhancement).

#### Troubleshooting
- “No results returned”: Verify `enable_search = true`, the index directory is writable, and fields you expect to read are marked `STORED` in the schema.
- “Deletes by path didn’t work”: Ensure the removed file path matches the stored `source_entry_path` exactly (case/normalization). The service watches `app_dir` recursively.
- “Data not visible immediately”: Allow for the commit interval, or add a `Flush` command in your client workflow.



Upsert Metadata:

```shell
([{'unique_id': <'test123'>, 'source': <'notes'>, 'uri': <'file:///usr/bin/gedit'>, 'title': <'how to win'>, 'subtitle': <'win over obstacles'>, 'description': <'this is my first note'>, 'keywords': <['kw1', 'kw2']>, 'icon': <'...'>, 'thumbnail': <'...'>, 'last_modified': <uint64 1763720442>, 'source_entry_path': <'/usr/share/applications/Alacritty.desktop'>, 'content': <'this is something very interesting book i am read'>, 'file_path': <'...'>}],)
```

Delete Metadata:

```shell
([<'test123'>, <'test124'>],)
```

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

```bash

$ https://github.com/mecha-org/mechanix-gui.git -b pre-release
$ cd services/search/server

$ cargo build --release

```

Run the server:

```

$ cargo run --release

```

Enable debug logging:

```

$ RUST_LOG=none,mxsearch=debug ../../target/release/mxsearch -s

```

## 🚀 Running MxSearch

```bash
$ RUST_LOG=none,mxsearch=debug,apps=debug,files=debug cargo run
```

---

### D-Bus Interface

MxSearch exposes its operations via **D-Bus**.

- **Bus name:** `org.mechanix.MxSearch`
- **Object path:** `/org/mechanix/MxSearch`
- **Interface:** `org.mechanix.MxSearch`

---

## ⚡ Methods

### ListApplications

List all installed applications

```bash
$ busctl call org.mechanix.MxSearch /org/mechanix/MxSearch org.mechanix.MxSearch ListApplications
```

### SearchApplications

```bash 
$ busctl call org.mechanix.MxSearch /org/mechanix/MxSearch org.mechanix.MxSearch SearchApplications s "<keyword>"
```

### SearchFiles

```bash
$ busctl call org.mechanix.MxSearch /org/mechanix/MxSearch org.mechanix.MxSearch SearchFiles s "<keyword>"
```

### SearchAppActions

```bash
$ busctl call org.mechanix.MxSearch /org/mechanix/MxSearch org.mechanix.MxSearch SearchAppActions s "<keyword>"
```

### SearchExternal

```bash
$ busctl --user call org.mechanix.MxSearch /org/mechanix/MxSearch org.mechanix.MxSearch SearchExternal s "<keyword>"
```

## 📋 TODOs

Apps

- [ ] Allow index applications from a custom directory ex: snap packages, flatpak packages.
- [ ] While parsing desktop entry, getting single value from multiple value fields. ex: categories, keywords