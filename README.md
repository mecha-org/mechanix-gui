# Mechanix GUI

This repository contains multiple components of Mechanix GUI developed in Rust and Flutter:

| Component    | GitHub Folder Link                                                                                                     |
| ------------ |------------------------------------------------------------------------------------------------------------------------|
| Launcher     | [mechanix-gui/shell/crates/launcher](https://github.com/mecha-org/mechanix-gui/tree/pre-release/shell/crates/launcher) |
| MXConf       | [mechanix-gui/services/conf](https://github.com/mecha-org/mechanix-gui/tree/pre-release/services/conf)                 |
| MXSearch     | [mechanix-gui/services/search](https://github.com/mecha-org/mechanix-gui/tree/pre-release/services/search)             |
| Files App    | [mechanix-gui/apps/files](https://github.com/mecha-org/mechanix-gui/tree/pre-release/apps/files)                       |
| Settings App | [mechanix-gui/apps/settings](https://github.com/mecha-org/mechanix-gui/tree/pre-release/apps/settings)                 |
| Music App    | [mechanix-gui/apps/music](https://github.com/mecha-org/mechanix-gui/tree/pre-release/apps/music)                       |
| Notes App    | [mechanix-gui/apps/notes](https://github.com/mecha-org/mechanix-gui/tree/pre-release/apps/notes)                       |

## Repository Structure

- **apps/** - Applications written in Flutter (files, settings, music, notes)
- **shell/** - Layer shell applications written in Rust (launcher, keyboard, notification)
- **services/** - Background services written in Rust (desktop, search, conf, system)
- **dbus/** - DBus client libraries written in Rust (freedesktop and mechanix)
- **shared/** - Shared libraries written in Rust

## Building and Running the GUI

### Run a specific binary crate, for example the launcher shell application

#### For running from root directory

```
cargo run -p mechanix-launcher
```

#### For running inside shell/crates/{package}

```
cargo run
```

#### Run the desktop services application

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
