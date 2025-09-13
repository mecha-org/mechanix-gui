# PulseAudio Sound Client

A Rust client module for interacting with PulseAudio sound server, providing high-level abstractions for audio device****
management and volume control.

## Overview

This client provides a safe and convenient interface to interact with the PulseAudio sound server, allowing
applications to manage audio devices, control volume levels, and handle audio streaming operations.

## Features
- **Device Management**: Enumerate, select, and manage audio devices.
- **Volume Control**: Adjust volume levels for sinks (output devices) and sources (input devices).

## Todo
- Implement support for audio streaming operations.
- Add support for PulseAudio event/signals notifications.
- Implement unit tests for critical components.

## Error Handling

The service uses a dedicated error type `PulseAudioError` that covers various failure scenarios:

- Connection and initialization errors
- Device management failures
- Message communication errors
- Server-related issues
- Generic operation failures

Each error type provides detailed context about the failure through the standard error trait implementation.

## Usage Examples
