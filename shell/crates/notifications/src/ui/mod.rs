use crate::widgets::NotificationList;
use gpui::{Context, IntoElement, Render, Window, *};

pub struct NotificationStory {
    pub notification_list: Entity<NotificationList>,
}

impl NotificationStory {
    pub fn new(notification_list: Entity<NotificationList>) -> Self {
        Self { notification_list }
    }
}

impl Render for NotificationStory {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .id("notification-story")
            .size_full()
            // Mount the list so its `render` runs and updates are visible
            .child(self.notification_list.clone())
        // .child(
        //     div()
        //         .id("close")
        //         .child(
        //             Icon::new(IconName::Close)
        //                 .size((px(20.), px(20.)))
        //                 .text_color(rgb(0xf4f4f4)),
        //         )
        //         .on_click(cx.listener(|this, _, window, cx| {
        //             let notification = Notification::info("Hello from info!");
        //             this.notification_list.update(cx, |list, cx| {
        //                 list.push(notification, window, cx);
        //             });
        //         }))
        // )
    }
}
