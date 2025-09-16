use bevy::prelude::*;
use notification::notification::Notification;
use service_plugins::notification::NotificationEvent;
use crate::components::cards::{Card, AppStackingState, create_card, get_stack_enitity_of_notification};
use crate::components::surface::{NotificationSurfaceEntity, NotificationStack, NotificationStacks};
use crate::components::app_name_row::spawn_app_name_row;
use crate::components::icon::save_notification_image_to_assets;

pub fn process_notifications(
    mut events: EventReader<NotificationEvent>,
    mut commands: Commands,
    drawing_surface: Option<Res<NotificationSurfaceEntity>>,
    mut stacks: ResMut<NotificationStacks>,
    asset_server: Res<AssetServer>,
    app_stacking: Res<AppStackingState>,
    stack_query: Query<&Children, With<NotificationStack>>,
    card_query: Query<Entity, With<Card>>
) {
    for event in events.read() {
        match event {
            NotificationEvent::Recieved(id, notification) => {
                info!(
                    "Notification received: id={:?}, application={:?}, expires={:?}",
                    &id,
                    &notification.app_name,
                    notification.get_expire_timeout()
                );
                let image_path = save_notification_image_to_assets(&id, &notification);
                let icon_handle = asset_server.load(image_path);
                if let Some(ref surface) = drawing_surface {
                    let app_stack = get_stack_enitity_of_notification(
                        surface,
                        notification.clone(),
                        &mut stacks,
                        &mut commands
                    );
                    create_card(
                        notification.clone(),
                        *id,
                        &mut commands,
                        app_stack,
                        icon_handle,
                        &app_stacking,
                        &stack_query,
                        &card_query
                    );
                }
            }
            NotificationEvent::Closed(id) => {
                info!("Notification closed: id={}", id);
            }
            _ => {}
        }
    }
}

