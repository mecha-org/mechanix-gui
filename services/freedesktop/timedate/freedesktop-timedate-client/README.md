# freedesktop-timedate-client

A Rust client library for interfacing with the freedesktop timedate1 D-Bus service, which manages system time, date, and timezone settings.

## Overview

This library provides a Rust interface to interact with the org.freedesktop.timedate1 D-Bus service. It allows you to manage system time settings, timezone configurations, and NTP (Network Time Protocol) synchronization on Linux systems.

## Features

- Query and set system time and date
- Manage system timezone settings
- Control NTP synchronization
- Asynchronous D-Bus communication using zbus
- Error handling with type safety

## Prerequisites

- Rust 1.85.1 or later
- Linux system with systemd and timedate1 service
- D-Bus system daemon

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
freedesktop-timedate-client = "0.1.0" # Use the latest version
tokio = { version = "1", features = ["full"] }
anyhow = "1"
```


### Available Methods

- `timezone()` - Get the current timezone
- `set_timezone(timezone)` - Set the system timezone
- `ntp_active()` - Check if NTP synchronization is active
- `set_ntp(active)` - Enable or disable NTP synchronization
- `set_time(usec_utc)` - Set the system time
- `local_rtc()` - Check if RTC is in local time
- `set_local_rtc(local)` - Configure RTC to use local time or UTC

## Error Handling

The library uses the `thiserror` crate for error handling and provides its own error type for specific D-Bus related errors.

# Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## See Also

- [systemd-timedate](https://www.freedesktop.org/software/systemd/man/org.freedesktop.timedate1.html) - Official documentation for the timedate1 D-Bus interface
- [D-Bus Specification](https://dbus.freedesktop.org/doc/dbus-specification.html) - D-Bus specification
