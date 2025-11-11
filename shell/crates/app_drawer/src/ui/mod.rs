use gpui::*;
use gpui::prelude::*;
use std::collections::BTreeMap;

use crate::models::{AppInfo, AppDrawerState};
use crate::ui::widgets::IconButton;
use crate::ui::icon::IconName;

pub mod icon;
mod widgets;

pub struct AppDrawer {
    pub state: AppDrawerState,
}

impl AppDrawer {
    pub fn new(state: AppDrawerState) -> Self {
        Self { state }
    }
}

impl Render for AppDrawer {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        use std::collections::BTreeMap;

        // Group apps by category
        let mut grouped: BTreeMap<String, Vec<AppInfo>> = BTreeMap::new();
        for app in self.state.apps.clone() {
            grouped.entry(app.category.clone()).or_default().push(app);
        }

        div()
            .flex()
            .flex_col()
            .bg(rgb(0x101010))
            .pt_12()
            .pl_8()
            .pr_8()
            .gap_4()
            .size_full()
            .child(
                div()
                    .flex_col()
                    .gap(px(24.))
                    .children(grouped.into_iter().map(|(category, apps)| {
                        div()
                            .flex_col()
                            .gap(px(32.))
                            .child("")
                            // Category app grid
                            .child(
                                div()
                                    .grid()
                                    .grid_cols(4)
                                    .gap(px(24.))
                                    .bg(rgb(0x181818))
                                    .p(px(20.))
                                    .rounded(px(12.))
                                    .children(apps.into_iter().enumerate().map(|(i, app)| {
                                        let app_name = app.name.clone();
                                        let app_icon = app.icon_path.clone();

                                        IconButton::new(("app", i))
                                            .icon(app_icon)
                                            .on_click(cx.listener(move |_, _, _, _| {
                                                println!("Launching app: {}", app_name);
                                            }))
                                    }))
                            )
                            // Category label below grid
                            // .child(format!("{}", category))
                            // .child(Label::new(category).color(rgb(0xCCCCCC)))
                        //    .child(
                        //      div()
                        //         .flex()
                        //         .justify_start()
                        //         .pl(px(8.)) // indent label slightly
                        //         .child(
                        //             Label::new(&category)
                        //                 .text_size(px(14.))
                        //                 .text_color(rgb(0xCCCCCC))
                        //                 .font_weight(FontWeight::MEDIUM),
                        //         ),
                        // )

                    }))
            )
            
            .child(
                div()
                    .flex()
                    .absolute()
                    .bottom(px(19.))
                    .left(px(210.))
                    .w(px(120.))
                    .h(px(4.))
                    .rounded(px(4.))
                    .bg(rgb(0x797979))
            )
    }
}
