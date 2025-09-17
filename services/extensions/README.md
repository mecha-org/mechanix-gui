# Mechanix Extension Service

A D-Bus service for detecting and managing extensions in the Mechanix GUI system.

## Overview

This service provides extension detection capabilities and exposes functionality through D-Bus interface for communication with other Mechanix components.

## Building

```bash
cargo build --release
```

## Running

```bash
cargo run
```

## Features

- Extension detection and management
- D-Bus interface for inter-process communication
- Graceful shutdown handling
- Comprehensive error handling and logging
