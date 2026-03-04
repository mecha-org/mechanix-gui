# 🎶 Music App

A lightweight and efficient music player application built with Flutter for Embedded Linux devices.

> **Your personal soundtrack for every moment!**

## Install Guide (Linux)

### Fedora

Install MPV and required libraries:

```bash
sudo dnf install mpv mpv-libs
```

### Debian

Install MPV development libraries:

```bash
sudo apt update
sudo apt install libmpv-dev
```

### Pre-requisites

- To install Flutter, follow this:
  [https://docs.flutter.dev/install](https://docs.flutter.dev/install)
- To install flutter-elinux, follow this:
  [https://github.com/sony/flutter-elinux](https://github.com/sony/flutter-elinux)

---

## Steps to Run Music App

1. **Clone the repository**

   ```bash
   git clone https://github.com/mecha-org/mechanix-gui.git
   cd apps/music
   ```

2. **Install dependencies**

   **For flutter-elinux**

   ```bash
   flutter-elinux pub get
   ```

   **For Flutter**

   ```bash
   flutter pub get
   ```

3. **Build & Run**

   **For flutter-elinux**

   ```bash
   flutter-elinux build
   flutter-elinux run
   ```

   **For Flutter**

   ```bash
   flutter build
   flutter run
   ```

---

## 🚀 Features

### 🎵 Core Playback Controls

- **Play/Pause**: Start and stop your music seamlessly
- **Next Track**: Skip to the following song
- **Previous Track**: Go back to the previous song
- **Progress Bar**: Visual timeline of current track
- **Add to Queue**: Add selected songs to a temporary playback queue

## 🎵 Music Features Implemented

- Play songs
- Pause playback
- Next track
- Previous track
- Song list view
- Add songs to favourites
- Remove songs from favourites
- Add songs to queue
- Queue-based playback priority
  - Queued songs play first
  - Automatically resumes normal playback after queue ends

---

## TODO

- Implementation of better search algorithm
- Add support for multiple artwork icons
