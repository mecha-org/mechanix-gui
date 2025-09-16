use bevy::prelude::*;
use notification::notification::Notification;

pub fn save_notification_image_to_assets(id: &u32, notification: &Notification) -> String {
    let notification_image_path = format!("assets/icons/{}.png", id.clone());
    if let Some(image) = notification.get_image() {
        image.save_to_path(notification_image_path.clone().into());
        info!("assets/icons/{}.png saved", &id);
    }
    let notification_image_path = format!("icons/{}.png", id.clone());
    notification_image_path
}