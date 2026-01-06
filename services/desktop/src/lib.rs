pub mod errors;
pub mod handlers;
pub mod interfaces;

// Re-export the Notification type for external users
pub use handlers::notification::Notification;