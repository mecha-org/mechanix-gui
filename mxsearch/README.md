# MxSearch

*A simple and fast search engine for your apps and files.*

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
=> Increase the watch file limit

```bash
sudo sysctl -w fs.inotify.max_user_watches=1048576
```

- **Crate: `files`**
    - Configure a dir to watch, and index files.
    - Allow free-form searching of indexed files.
    - Configure searchable fields in a settings file.
        - searchable_fields = [
          "file_type",
          "name",
          "content"
          ]
    - Graceful shutdown support (e.g. optional task cancellation).
    - Load existing entries from the provided directory.
    - We can configure the depth of the directory to be indexed.
    - We can configure the allowed extensions to be indexed.
    - We can configure the max size of the file content to be indexed.

Settings file example:

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
```

- **Configurable File Search Service**
    - Enable/disable file search.
    - Define the path to the files you want to include in the search.

---

## 🛠️ Configuration Example (`config.toml`)

```toml
[general]

[apps]
enable_search_apps = true
apps_dir = "/usr/share/applications"
index_dir = "<INDEX_DIR>"
searchable_fields = [
    "name",
    "genericname",
    "comment",
    "keywords",
    "categories"
]
```

## 🚀 Running MxSearch

```bash
 RUST_LOG=none,mxsearch=debug,apps=debug,files=debug cargo run
```

## TODOs

Apps

- [ ] Allow index applications from a custom directory ex: snap packages, flatpak packages.
- [ ] While parsing desktop entry, getting single value from multiple value fields. ex: categories, keywords

Files

- [ ] Watch dir limit reached for /home -> we have to increase the watch limit.