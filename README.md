# Mechanix GUI

This repository contains multiple components of Mechanix GUI developed in Rust and Flutter:

- **apps/** - Applications written in Flutter (settings, files, camera, notes)
- **shell/** - Layer shell applications written in Rust (launcher, keyboard, notification)
- **services/** - Background services written in Rust (desktop, search, conf, system)
- **dbus/** - DBus client libraries written in Rust (freedesktop and mechanix)
- **tools/** - Tools
- **utils/** - Utility libraries written in Rust
- **shared/** - Shared libraries written in Rust

## Building and Running

## Rust apps

Run Rust builds with Cargo workspace commands:

Build all Rust crates in the workspace
```
cargo build
```

Run a specific binary crate, for example the launcher shell application
```
cargo run -p mechanix-launcher
```

Run the desktop services application
```
cargo run -p mechanix-desktop-services
```

## Flutter apps

To run Flutter applications, navigate to the desired app folder and use Flutter Elinux commands:

```
cd apps/settings
flutter-elinux pub get
flutter-elinux run
```

Replace `settings` with the name of the app you want to work on.