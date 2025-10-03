# Mechanix GUI

This repository contains multiple components of Mechanix GUI developed in Rust.

- **apps/** - Applications written using mctk, gui library in Rust (settings, files, camera)
- **shell/** - Shell applications written using mctk, gui library in Rust (launcher, keyboard)
- **services/** - Background services written in Rust (desktop, system)
- **commons/** - DBUS client's and utils written in Rust which are used in both apps and shell

## Building and Running

### Follow these steps to run apps or shell applications, similar to the launcher setup:

**1. Navigate to the directory:**

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