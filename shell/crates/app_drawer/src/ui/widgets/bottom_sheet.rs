use std::path::PathBuf;

use crate::ui::widgets::TextButton;

use gpui::*;
use icons::prelude::Icons;
use theme::prelude::Theme;

const DIVIDER_WIDTH: f32 = 508.0;

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
        .w(px(DIVIDER_WIDTH))
        .h(px(1.0))
        .bg(colors.background_700)
        .id("divider")
}

impl AppDrawer {
    pub fn render_main_sheet(&self, cx: &mut gpui::Context<AppDrawer>) -> AnyElement {
        let colors = Theme::global(cx).colors.clone();
        let icons = Icons::global(cx).app_drawer.clone();

        let app = match &self.sheet_app {
            Some(a) => a,
            None => {
                return Empty.into_any();
            }
        };
        let icon_path = app.icon_path.clone();
        let icon = Self::resolved_icon(&icon_path, cx);

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
                        div()
                            .id("app-icon")
                            .bg(Theme::global(cx).colors.background_700)
                            .flex()
                            .items_center()
                            .justify_center()
                            .rounded(px(5.6))
                            .size(px(60.0))
                            .child(
                                div()
                                    .w(px(40.0))
                                    .h(px(40.0))
                                    .border(px(1.0))
                                    .items_center()
                                    .justify_center()
                                    .child(icon),
                            ),
                    )
                    .child(
                        div()
                            .text_size(px(20.0))
                            .text_color(colors.foreground_300)
                            .line_height(px(1.2))
                            .text_ellipsis()
                            .max_w(px(200.0))
                            .child(app.name.clone()),
                    ),
            )
            .child(div().flex().flex_col().gap(px(12.0)).children([
                divider(cx),
                self.sheet_row(
                    "Search a file",
                    icons.search,
                    colors.foreground_300,
                    cx.listener(|_this: &mut AppDrawer, _, _, _cx| {}),
                ),
                self.sheet_row(
                    "Properties",
                    icons.info,
                    colors.foreground_300,
                    cx.listener(|this: &mut AppDrawer, _, _, cx| {
                        this.sheet_kind = BottomSheetKind::Properties;
                        cx.notify();
                    }),
                ),
                self.sheet_row(
                    "Delete app",
                    icons.delete,
                    rgb(0xffff0000),
                    cx.listener(|this: &mut AppDrawer, _, _, cx| {
                        this.sheet_kind = BottomSheetKind::ConfirmDelete;
                        cx.notify();
                    }),
                ),
            ]))
            .into_any()
    }

    pub fn sheet_row(
        &self,
        label: &str,
        icon: PathBuf,
        text_color: Rgba,
        on_click: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
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
                    .child(
                        svg()
                            .size_full()
                            .external_path(SharedString::from(icon.to_string_lossy().to_string()))
                            .text_color(text_color),
                    ),
            )
            .child(
                div()
                    .text_size(px(16.0))
                    .line_height(px(1.2))
                    .text_color(text_color)
                    .font_weight(FontWeight::NORMAL)
                    .child(row_label),
            )
    }

    pub fn render_delete_sheet(&self, cx: &mut Context<AppDrawer>) -> AnyElement {
        let colors = Theme::global(cx).colors.clone();

        let app = match &self.sheet_app {
            Some(a) => a,
            None => {
                return Empty.into_any();
            }
        };
        let icon_path = app.icon_path.clone();
        let icon = Self::resolved_icon(&icon_path, cx);

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
                        div()
                            .id("app-icon")
                            .bg(Theme::global(cx).colors.background_800)
                            .flex()
                            .items_center()
                            .justify_center()
                            .rounded(px(5.6))
                            .size(px(60.0))
                            .child(
                                div()
                                    .w(px(40.0))
                                    .h(px(40.0))
                                    .border(px(1.0))
                                    .items_center()
                                    .justify_center()
                                    .child(icon),
                            ),
                    )
                    .child(
                        div()
                            .text_size(px(20.0))
                            .text_color(colors.foreground_300)
                            .line_height(px(1.2))
                            .max_w(px(200.0))
                            .text_ellipsis()
                            .child(app.name.clone()),
                    ),
            )
            .child(
                div().flex_col().gap(px(16.0)).children([
                    // Title
                    div()
                        .text_size(px(20.0))
                        .text_color(colors.foreground_200)
                        .font_weight(FontWeight::BOLD)
                        .child(format!("Delete ‘{}’ app?", app.name)),
                    // Subtitle
                    div()
                        .mt(px(10.0))
                        .text_size(px(16.0))
                        .text_color(colors.foreground_200)
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
                                .rounded(px(8.0))
                                .bg_color(colors.background_500)
                                .text_color(colors.foreground_200)
                                .text_size(px(18.0))
                                .on_click(cx.listener(|this: &mut AppDrawer, _, _, cx| {
                                    this.sheet_kind = BottomSheetKind::MainOptions;
                                    cx.notify();
                                })),
                            // DELETE
                            TextButton::new("confirm-delete", "Delete app")
                                .width(px(246.0))
                                .height(px(40.0))
                                .rounded(px(8.0))
                                .text_size(px(18.0))
                                .bg_color(rgb(0xffd3002a)) // Red
                                .text_color(gpui::white())
                                .text_color(colors.foreground_200)
                                .on_click(cx.listener(|this: &mut AppDrawer, _, _, cx| {
                                    // if let Some(app) = &this.sheet_app {
                                    //     this.remove_app(&app.name);
                                    // }
                                    this.show_bottom_sheet = false;
                                    cx.notify();
                                })),
                        ]),
                ]),
            )
            .into_any()
    }

    pub fn render_properties_sheet(&self, cx: &mut Context<AppDrawer>) -> AnyElement {
        let app = match &self.sheet_app {
            Some(a) => a,
            None => {
                return Empty.into_any();
            }
        };
        let icon = Self::resolved_icon(&app.icon_path, cx);
        let colors = Theme::global(cx).colors.clone();

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
                    .child(
                        div()
                            .id("app-icon")
                            .bg(Theme::global(cx).colors.background_800)
                            .flex()
                            .items_center()
                            .justify_center()
                            .rounded(px(5.6))
                            .size(px(60.0)) // outer size
                            .child(
                                div()
                                    .w(px(40.0))
                                    .h(px(40.0))
                                    .border(px(1.0))
                                    .items_center()
                                    .justify_center()
                                    .child(icon),
                            ),
                    )
                    .child(
                        div()
                            .text_size(px(20.0))
                            .text_color(colors.foreground_300)
                            .font_weight(FontWeight::BOLD)
                            .text_ellipsis()
                            .max_w(px(200.0))
                            .child(app.name.clone()),
                    ),
                {
                    let kind = app
                        .categories
                        .first()
                        .and_then(|s| {
                            let t = s.trim();
                            if t.is_empty() { None } else { Some(t) }
                        })
                        .unwrap_or("Application");

                    self.properties_row("Kind", kind, colors.foreground_400)
                        .mt_5()
                },
                self.properties_row("Size", "20 mb on disk", colors.foreground_400),
                self.properties_row("Location", app.app_path.as_str(), colors.foreground_400),
                self.properties_row("Accessed", "25-12-2025, 12.30pm", colors.foreground_400),
                self.properties_row("Created", "25-12-2025, 12.30pm", colors.foreground_400),
                self.properties_row("Modified", "25-12-2025, 12.30pm", colors.foreground_400),
            ])
            .into_any()
    }

    pub fn properties_row(&self, label: &str, value: &str, text_color: Rgba) -> Stateful<Div> {
        let row_label: SharedString = label.to_string().into();
        let id: SharedString = label.to_string().into();

        div()
            .id(id)
            .flex()
            .flex_row()
            .items_start()
            .justify_between()
            .child(
                div()
                    .text_size(px(18.0))
                    .text_color(text_color)
                    .child(row_label),
            )
            .child(
                div()
                    .flex()
                    .justify_end()
                    .text_size(px(18.0))
                    .text_color(text_color)
                    .child(value.to_string())
                    .w(px(200.0))
                    .text_ellipsis(),
            )
    }
}
