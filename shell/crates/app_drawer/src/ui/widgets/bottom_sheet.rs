use crate::prelude::Icon;
use crate::prelude::IconName;
use crate::ui::utils::prelude::DesktopApp;
use crate::ui::widgets::{IconButton, TextButton};
use gpui::*;

use crate::prelude::AppDrawer;

#[derive(Clone, Debug)]
pub enum BottomSheetKind {
    None,
    MainOptions,
    ConfirmDelete,
    Properties,
}

pub fn divider() -> Stateful<Div> {
    div().w(px(508.)).h(px(1.)).bg(rgb(0x404040)).id("divider")
}

impl AppDrawer {
    pub fn render_main_sheet(&self, cx: &mut gpui::Context<AppDrawer>) -> AnyElement {
        let app = match &self.sheet_app {
            Some(a) => a,
            None => return Empty.into_any(),
        };
        let icon = DesktopApp::resolved_icon(&app.icon_path);

        div()
            .flex_col()
            .gap(px(16.))
            .children([
                // Row: Icon + App Name
                div()
                    .id("app-name")
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(12.))
                    .items_center()
                    .pt(px(8.))
                    .pb(px(8.))
                    .child(
                        IconButton::new("app-icon")
                            .icon(icon)
                            .width(px(48.))
                            .height(px(48.)),
                    )
                    .child(
                        div()
                            .text_size(px(20.))
                            .text_color(rgb(0xFFFFFF))
                            .font_weight(FontWeight::BOLD)
                            .child(app.name.clone()),
                    ),
                // More rows (e.g., search, delete, etc.)
                divider(),
                self.sheet_row(
                    "Search a file",
                    IconName::Search,
                    gpui::white(),
                    cx.listener(|_this: &mut AppDrawer, _, _, _cx| {}),
                ),
                divider(),
                self.sheet_row(
                    "Properties",
                    IconName::Info,
                    gpui::white(),
                    cx.listener(|this: &mut AppDrawer, _, _, cx| {
                        this.sheet_kind = BottomSheetKind::Properties;
                        cx.notify();
                    }),
                ),
                divider(),
                self.sheet_row(
                    "Delete app",
                    IconName::Delete,
                    gpui::red(),
                    cx.listener(|this: &mut AppDrawer, _, _, cx| {
                        this.sheet_kind = BottomSheetKind::ConfirmDelete;
                        cx.notify();
                    }),
                ),
                div().id("button").mt(px(18.)).mb(px(-10.)).child(
                    TextButton::new("close-btn", "Close")
                        .width(px(508.))
                        .height(px(40.))
                        .rounded(px(18.))
                        .bg_color(rgb(0x424242))
                        .text_color(rgb(0xFFFFFF))
                        .on_click(cx.listener(|this: &mut AppDrawer, _, _, cx| {
                            this.show_bottom_sheet = false;
                            cx.notify();
                        })),
                ),
            ])
            .into_any()
    }

    pub fn sheet_row(
        &self,
        label: &str,
        icon: IconName,
        text_color: Hsla,
        on_click: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Stateful<Div> {
        let row_label: SharedString = label.to_string().into();
        let id: SharedString = label.to_string().into();

        div()
            .id(id)
            .flex()
            .flex_row()
            .gap(px(12.))
            .items_center()
            .pt(px(8.))
            .pb(px(8.))
            .on_click(on_click)
            .child(
                div()
                    .w(px(20.0))
                    .h(px(20.0))
                    .items_center()
                    .justify_center()
                    .mr(px(8.0))
                    .ml(px(16.0))
                    .flex()
                    .child(Icon::build(icon).text_color(text_color)),
            )
            .child(
                div()
                    .text_size(px(16.))
                    .text_color(text_color)
                    .font_weight(FontWeight::BOLD)
                    .child(row_label),
            )
    }

    pub fn render_delete_sheet(&self, cx: &mut Context<AppDrawer>) -> AnyElement {
        let app = match &self.sheet_app {
            Some(a) => a,
            None => return Empty.into_any(),
        };

        div()
            .flex_col()
            .gap(px(16.))
            .children([
                // Title
                div()
                    .text_size(px(20.))
                    .text_color(rgb(0xFFFFFF))
                    .font_weight(FontWeight::BOLD)
                    .child(format!("Delete ‘{}’", app.name)),
                // Subtitle
                div()
                    .mt(px(10.))
                    .text_size(px(16.))
                    .text_color(rgb(0xC0C0C0))
                    .child("This action will delete the app permanently"),
                // Buttons row
                div()
                    .flex()
                    .flex_row()
                    .gap(px(12.))
                    .mt(px(18.))
                    .mb(px(-10.))
                    .children([
                        // Cancel
                        TextButton::new("cancel-delete", "Cancel")
                            .width(px(246.))
                            .height(px(40.))
                            .rounded(px(18.))
                            .bg_color(rgb(0x505050))
                            .text_color(gpui::white())
                            .text_size(px(16.))
                            .on_click(cx.listener(|this: &mut AppDrawer, _, _, cx| {
                                this.sheet_kind = BottomSheetKind::MainOptions;
                                cx.notify();
                            })),
                        // DELETE
                        TextButton::new("confirm-delete", "Delete app")
                            .width(px(246.))
                            .height(px(40.))
                            .rounded(px(18.))
                            .text_size(px(16.))
                            .bg_color(rgb(0xC92A2A)) // Red
                            .text_color(gpui::white())
                            .on_click(cx.listener(|this: &mut AppDrawer, _, _, cx| {
                                // if let Some(app) = &this.sheet_app {
                                //     this.remove_app(&app.name);
                                // }
                                this.show_bottom_sheet = false;
                                cx.notify();
                            })),
                    ]),
            ])
            .into_any()
    }

    pub fn render_properties_sheet(&self, cx: &mut Context<AppDrawer>) -> AnyElement {
        let app = match &self.sheet_app {
            Some(a) => a,
            None => return Empty.into_any(),
        };
        let icon = DesktopApp::resolved_icon(&app.icon_path);

        div()
            .flex_col()
            .gap(px(16.))
            .children([
                div()
                    .id("app-name")
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(12.))
                    .items_center()
                    .pt(px(8.))
                    .pb(px(8.))
                    .child(
                        IconButton::new("app-icon")
                            .icon(icon)
                            .width(px(48.))
                            .height(px(48.)),
                    )
                    .child(
                        div()
                            .text_size(px(20.))
                            .text_color(rgb(0xFFFFFF))
                            .font_weight(FontWeight::BOLD)
                            .child(app.name.clone()),
                    ),
                divider(),
                self.properties_row("Version", "1.0.1", gpui::white()),
                divider(),
                self.properties_row("Size", "200 MB", gpui::white()),
                div().id("button").mt(px(18.)).mb(px(-10.)).child(
                    TextButton::new("close-btn", "Close")
                        .width(px(508.))
                        .height(px(40.))
                        .rounded(px(18.))
                        .bg_color(rgb(0x424242))
                        .text_color(rgb(0xFFFFFF))
                        .on_click(cx.listener(|this: &mut AppDrawer, _, _, cx| {
                            this.show_bottom_sheet = false;
                            cx.notify();
                        })),
                ),
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
            .items_center()
            .justify_between()
            .pt(px(8.))
            .pb(px(8.))
            .pl(px(16.))
            .pr(px(16.))
            .child(
                div()
                    .text_size(px(16.))
                    .text_color(text_color)
                    .font_weight(FontWeight::BOLD)
                    .child(row_label),
            )
            .child(
                div()
                    .text_size(px(14.))
                    .text_color(text_color)
                    .font_weight(FontWeight::MEDIUM)
                    .child(value.to_string()),
            )
    }
}
