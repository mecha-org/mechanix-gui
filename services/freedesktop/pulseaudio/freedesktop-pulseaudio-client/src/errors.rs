//! Represents errors that can occur when interacting with PulseAudio.
//!
//! This module provides a comprehensive set of error types that may occur during
//! PulseAudio operations, such as volume control, device management, and audio streaming.

use crate::service::{PulseInitError, PulseServerError};

/// Represents various errors that can occur during PulseAudio operations.
///
/// This enum encapsulates different types of errors that may arise while
/// interacting with the PulseAudio freedesktop-pulseaudio-client server, such as connection issues,
/// device management failures, or invalid operations.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum PulseAudioError {
    /// A generic, unspecified error occurred during PulseAudio operations.
    ///
    /// This variant is used when a more specific error classification is not available
    /// or when the underlying error doesn't fit into other specific categories.
    #[error("generic error")]
    Generic,

    /// An error occurred while initializing pulseaudio service.
    #[error("failed to init pulseaudio service: {0}")]
    InitPulseAudioServiceError(String),

    /// An error occurred while trying to send results back to the sender.
    #[error("failed to send request back to sender: {0}")]
    SendRequestError(String),

    /// An error occurred while trying to initialize the pulse handle.
    #[error("failed to init pulse handle: {0}")]
    PulseInitError(#[from] PulseInitError),

    /// An error originating from the pulse handle layer.
    #[error("pulse handle error: {0}")]
    PulseServerError(#[from] PulseServerError),

    /// An error occurred while trying to send a message to the PulseAudio server.
    #[error("failed to send message to PulseAudio server: {0}")]
    SendMessageError(String),

    /// An error occurred while trying to receive a message from the PulseAudio server.
    #[error("the received message from PulseAudio server is unexpected")]
    UnexpectedMessage,

    /// An error occurred while trying to receive a message from the PulseAudio server.
    #[error("failed to receive message from PulseAudio server")]
    ReceiveMessageError,
}
