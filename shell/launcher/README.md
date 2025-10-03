# Mechanix Launcher 2.0

A shell application for interacting with system apps, installed apps, and system monitoring tools, built with MCTK (a GUI library in Rust).

# [![Launcher](https://github.com/user-attachments/assets/5479f1ea-1938-4640-acc1-48ac1f99d5d4)](https://github.com/mecha-org/mechanix-gui)

## Overview

Mechanix Launcher provides interface for managing your system, offering quick access to essential apps and real-time system information through widgets.

## Features

### Status Bar

The status bar remains visible at the top of the screen, providing access to critical system information:

- **Time** - Current system time
- **Wireless Status** - Wireless connectivity indicator
- **Bluetooth Status** - Bluetooth connection state
- **Battery Status** - Remaining battery percentage and charging indicator

### Home Screen

The home screen serves as your primary interface, featuring:

#### Widgets

- **Memory Widget** - Real-time memory usage monitoring
- **News Widget** - Latest technology news updates

#### Pinned Applications

Quick access to frequently used applications:

1. Files
2. Notes
3. Music
4. Settings
5. Terminal
6. Chromium
7. Firefox

## Getting Started

### Running in Debug Mode

**1. Navigate to the launcher directory:**

```bash
cd mechanix-gui/shell/launcher
```

**2. Configure settings:**

Copy the example configuration file:

```bash
cp settings.yml.example settings.yml
```

**3. Update asset paths:**

Open `settings.yml` and modify the paths to match your local directory structure.

**Example:**

Change:
```yaml
/usr/share/mechanix/shell/launcher/assets/icons/terminal_icon.png
```

To:
```yaml
./src/assets/icons/terminal_icon.png
```

> **Note:** Update all asset paths (fonts, icons, and other resources) to be relative to your project directory.

**4. Run the application:**

```bash
cargo run
```

## Requirements

- Rust
- Linux-based operating system