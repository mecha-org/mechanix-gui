use crate::prelude::Icon;
use crate::prelude::IconName;
use crate::ui::utils::prelude::DesktopApp;
use crate::ui::widgets::{ IconButton, TextButton };

use gpui::*;
use theme::prelude::Theme;

const GRID_ROW_WIDTH: f32 = 420.0;

use crate::prelude::AppDrawer;

#[derive(Clone, Debug)]
pub enum BottomSheetKind {
    None,
    MainOptions,
    ConfirmDelete,
    Properties,
}

pub fn divider(cx: &mut gpui::Context<AppDrawer>) -> Stateful<Div> {
    let colors = Theme::global(cx).colors.clone();

    div()
        .flex()
        .items_center()
        .justify_center()
        .mt(px(10.0))
        .w(px(GRID_ROW_WIDTH))
        .h(px(1.0))
        .bg(colors.background_700)
        .id("divider")
}

impl AppDrawer {
    pub fn render_main_sheet(&self, cx: &mut gpui::Context<AppDrawer>) -> AnyElement {
        let colors = Theme::global(cx).colors.clone();

        let app = match &self.sheet_app {
            Some(a) => a,
            None => {
                return Empty.into_any();
            }
        };
        let icon = DesktopApp::resolved_icon(&app.icon_path);

        div()
            .flex()
            .flex_col()
            .h_full()
            .child(
                div()
                    .id("app-name")
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(12.0))
                    .items_center()
                    .pt(px(8.0))
                    .pb(px(8.0))
                    .child(
                        IconButton::new("app-icon")
                            .icon(icon)
                            .size(px(60.0))
                            .width(px(40.0))
                            .height(px(40.0))
                    )
                    .child(
                        div()
                            .text_size(px(20.0))
                            .text_color(colors.foreground_300)
                            .line_height(px(1.2))
                            .child(app.name.clone())
                    )
            )

            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(12.0))
                    .children([
                        divider(cx),

                        self.sheet_row(
                            "Search a file",
                            IconName::Search,
                            colors.foreground_300,
                            cx.listener(|_this: &mut AppDrawer, _, _, _cx| {})
                        ),
                        self.sheet_row(
                            "Properties",
                            IconName::Info,
                            colors.foreground_300,
                            cx.listener(|this: &mut AppDrawer, _, _, cx| {
                                this.sheet_kind = BottomSheetKind::Properties;
                                cx.notify();
                            })
                        ),
                        self.sheet_row(
                            "Delete app",
                            IconName::Delete,
                            rgb(0xffff0000),
                            cx.listener(|this: &mut AppDrawer, _, _, cx| {
                                this.sheet_kind = BottomSheetKind::ConfirmDelete;
                                cx.notify();
                            })
                        ),
                    ])
            )
            .into_any()
    }

    pub fn sheet_row(
        &self,
        label: &str,
        icon: IconName,
        text_color: Rgba,
        on_click: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static
    ) -> Stateful<Div> {
        let row_label: SharedString = label.to_string().into();
        let id: SharedString = label.to_string().into();

        div()
            .id(id)
            .flex()
            .flex_row()
            .items_start()
            .justify_start()
            .gap(px(12.0))
            .items_center()
            .h(px(36.0))
            .pt(px(8.0))
            .pb(px(8.0))
            .on_click(on_click)
            .child(
                div()
                    .w(px(20.0))
                    .h(px(20.0))
                    .items_center()
                    .justify_center()
                    .mr(px(8.0))
                    .flex()
                    .child(Icon::build(icon).text_color(text_color))
            )
            .child(
                div()
                    .text_size(px(16.0))
                    .line_height(px(1.2))
                    .text_color(text_color)
                    .font_weight(FontWeight::NORMAL)
                    .child(row_label)
            )
    }

    pub fn render_delete_sheet(&self, cx: &mut Context<AppDrawer>) -> AnyElement {
        let app = match &self.sheet_app {
            Some(a) => a,
            None => {
                return Empty.into_any();
            }
        };

        div()
            .flex_col()
            .gap(px(16.0))
            .children([
                // Title
                div()
                    .text_size(px(20.0))
                    .text_color(rgb(0xffffff))
                    .font_weight(FontWeight::BOLD)
                    .child(format!("Delete ‘{}’", app.name)),
                // Subtitle
                div()
                    .mt(px(10.0))
                    .text_size(px(16.0))
                    .text_color(rgb(0xc0c0c0))
                    .child("This action will delete the app permanently"),
                // Buttons row
                div()
                    .flex()
                    .flex_row()
                    .gap(px(12.0))
                    .mt(px(18.0))
                    .mb(px(-10.0))
                    .children([
                        // Cancel
                        TextButton::new("cancel-delete", "Cancel")
                            .width(px(246.0))
                            .height(px(40.0))
                            .rounded(px(18.0))
                            .bg_color(rgb(0x505050))
                            .text_color(gpui::white())
                            .text_size(px(16.0))
                            .on_click(
                                cx.listener(|this: &mut AppDrawer, _, _, cx| {
                                    this.sheet_kind = BottomSheetKind::MainOptions;
                                    cx.notify();
                                })
                            ),
                        // DELETE
                        TextButton::new("confirm-delete", "Delete app")
                            .width(px(246.0))
                            .height(px(40.0))
                            .rounded(px(18.0))
                            .text_size(px(16.0))
                            // .bg_color(rgb(0xc92a2a)) // Red
                            .text_color(gpui::white())
                            .on_click(
                                cx.listener(|this: &mut AppDrawer, _, _, cx| {
                                    // if let Some(app) = &this.sheet_app {
                                    //     this.remove_app(&app.name);
                                    // }
                                    this.show_bottom_sheet = false;
                                    cx.notify();
                                })
                            ),
                    ]),
            ])
            .into_any()
    }

    pub fn render_properties_sheet(&self, cx: &mut Context<AppDrawer>) -> AnyElement {
        let app = match &self.sheet_app {
            Some(a) => a,
            None => {
                return Empty.into_any();
            }
        };
        let icon = DesktopApp::resolved_icon(&app.icon_path);

        div()
            .flex_col()
            .gap(px(16.0))
            .children([
                div()
                    .id("app-name")
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(12.0))
                    .items_center()
                    .pt(px(8.0))
                    .pb(px(8.0))
                    .child(IconButton::new("app-icon").icon(icon).width(px(48.0)).height(px(48.0)))
                    .child(
                        div()
                            .text_size(px(20.0))
                            .text_color(rgb(0xffffff))
                            .font_weight(FontWeight::BOLD)
                            .child(app.name.clone())
                    ),
                divider(cx),
                self.properties_row("Version", "1.0.1", gpui::white()),
                // divider(cx),
                self.properties_row("Size", "200 MB", gpui::white()),
                // div()
                //     .id("button")
                //     .mt(px(18.0))
                //     .mb(px(-10.0))
                //     .child(
                //         TextButton::new("close-btn", "Close")
                //             .width(px(508.0))
                //             .height(px(40.0))
                //             .rounded(px(18.0))
                //             .bg_color(rgb(0x424242))
                //             .text_color(rgb(0xffffff))
                //             .on_click(
                //                 cx.listener(|this: &mut AppDrawer, _, _, cx| {
                //                     this.show_bottom_sheet = false;
                //                     cx.notify();
                //                 })
                //             )
                //     ),
            ])
            .into_any()
    }

    pub fn properties_row(&self, label: &str, value: &str, text_color: Hsla) -> Stateful<Div> {
        let row_label: SharedString = label.to_string().into();
        let id: SharedString = label.to_string().into();

        div()
            .id(id)
            .flex()
            .flex_row()
            .items_start()
            .justify_between()
            .pt(px(8.0))
            .pb(px(8.0))
            .pl(px(16.0))
            .pr(px(16.0))
            .child(
                div()
                    .text_size(px(16.0))
                    .text_color(text_color)
                    .font_weight(FontWeight::BOLD)
                    .child(row_label)
            )
            .child(
                div()
                    .text_size(px(14.0))
                    .text_color(text_color)
                    .font_weight(FontWeight::MEDIUM)
                    .child(value.to_string())
            )
    }
}
