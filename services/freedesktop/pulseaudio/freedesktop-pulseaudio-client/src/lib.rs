//! PulseAudio integration library for managing audio devices and controls.
//!
//! This library provides a high-level interface for interacting with PulseAudio,
//! allowing applications to manage audio devices, control volume levels, and
//! handle audio-related operations asynchronously.

use crate::errors::PulseAudioError;
use async_trait::async_trait;

pub mod errors;
pub mod service;
