use sled::Db;
use crate::notification::Notification;
use std::collections::HashMap;
use serde_json;

/// Database file constant: Path where the database will be stored.
pub const DATABASE_PATH: &str = "notifications.db";

/// adds notification to database (notification.db)
/// `if let Err(e) = add_notification_to_db(id, &notification).await { eprintln!("Failed to add notification to database: {}", e);}`
pub async fn add_notification_to_db(
    id: u32,
    notification: &Notification
) -> Result<(), Box<dyn std::error::Error>> {
    let db = sled::open(DATABASE_PATH)?;
    let notification_bytes = serde_json::to_vec(notification)?;
    db.insert(id.to_be_bytes(), notification_bytes)?;
    db.flush()?;
    Ok(())
}

/// removes notification from database (notification.db)
/// `if let Err(e) = remove_notification_from_db(id).await { eprintln!("Failed to remove notification from database: {}", e);}`
pub async fn remove_notification_from_db(id: u32) -> Result<bool, Box<dyn std::error::Error>> {
    let db = sled::open(DATABASE_PATH)?;
    match db.remove(id.to_be_bytes())? {
        Some(_) => {
            db.flush()?;
            Ok(true)
        }
        None => Ok(false),
    }
}

/// Get all notifications from database
pub async fn get_all_notifications_from_db() -> Result<
    HashMap<u32, Notification>,
    Box<dyn std::error::Error>
> {
    let db = sled::open(DATABASE_PATH)?;
    let mut notifications = HashMap::new();

    for result in db.iter() {
        let (key, value) = result?;
        /// Convert key from big-endian bytes back to u32
        let id_bytes: [u8; 4] = key
            .as_ref()
            .try_into()
            .map_err(|_| "Invalid key length")?;
        let id = u32::from_be_bytes(id_bytes);

        /// Deserialize notification from JSON
        let notification: Notification = serde_json::from_slice(&value)?;
        notifications.insert(id, notification);
    }
    db.flush()?;
    Ok(notifications)
}