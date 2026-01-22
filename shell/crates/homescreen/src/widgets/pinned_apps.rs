use std::{path::PathBuf, pin};

use crate::{widgets::HomescreenWidget, PinnedAppsState};
use dispatcher::Dispatcher;
use gpui::{prelude::FluentBuilder, *};
use theme::{ActiveFonts, ActiveTheme};

pub struct PinnedApps {
    bounds: Bounds<Pixels>,
    apps: Vec<PathBuf>,
    background_color: Hsla,
    border_color: Hsla,
    has_border: bool,
}

impl PinnedApps {
    pub fn new(
        apps: impl Into<Vec<PathBuf>>,
        color: impl Into<Hsla>,
        border_color: impl Into<Hsla>,
        has_border: bool,
    ) -> Self {
        Self {
            bounds: Bounds::default(),
            apps: apps.into(),
            background_color: color.into(),
            border_color: border_color.into(),
            has_border,
        }
    }

    pub fn on_app_click(possible_app_id: String, exec: String, cx: &mut App) {
        let sender = Dispatcher::global(cx).0.clone();
        cx.background_executor()
            .spawn(async move {
                _ = sender
                    .broadcast(dispatcher::Message::LaunchApp {
                        app_id: possible_app_id,
                        exec,
                    })
                    .await;
            })
            .detach();
    }
}

impl Default for PinnedApps {
    fn default() -> Self {
        Self::new(vec![], rgb(0x4ecdc4), rgb(0x45b7aa), true)
    }
}

impl HomescreenWidget for PinnedApps {
    fn render(&self, cx: &mut gpui::App) -> gpui::AnyElement {
        let primary = cx.fonts().primary.clone();
        let colors = cx.theme().colors.clone();
        let text_color = colors.accent_200.clone();
        let pinned_apps = PinnedAppsState::global(cx).apps.clone();
        let icon_bg_color = colors.background_600.clone();

        div()
            .size_full()
            .flex()
            .justify_center()
            .items_center()
            .child(
                div()
                    .absolute()
                    .top(px(8.))
                    .left(px(12.))
                    .text_size(px(20.))
                    .font_family(primary)
                    .text_color(text_color)
                    .child("Apps"),
            )
            .child(
                div()
                    .absolute()
                    .top(px(48.))
                    .left(px(24.))
                    .w(px(460.))
                    .h(px(172.))
                    .grid()
                    .grid_cols(5)
                    .grid_rows(2)
                    .gap(px(20.))
                    .children(pinned_apps.iter().enumerate().map(|(idx, app)| {
                        let app_id = app.possible_app_id.clone();
                        let exec = app.exec.clone();
                        div()
                            .id(idx)
                            .bg(icon_bg_color)
                            .size(px(76.))
                            .rounded(px(7.6))
                            .flex()
                            .items_center()
                            .justify_center()
                            .cursor_pointer()
                            .active(|this| this.bg(colors.background_700))
                            .on_click(move |_, _, cx| {
                                Self::on_app_click(app_id.clone(), exec.clone(), cx);
                            })
                            .when_some(app.icon_path.clone(), |this, icon| {
                                this.child(
                                    div()
                                        .size(px(49.5))
                                        .flex()
                                        .items_center()
                                        .justify_center()
                                        .child(img(PathBuf::from(icon)).size_full()),
                                )
                            })
                    })),
            )
            .into_any_element()
    }

    fn set_bounds(&mut self, bounds: Bounds<Pixels>) {
        self.bounds = bounds;
    }

    fn get_bounds(&self) -> Bounds<Pixels> {
        self.bounds
    }

    fn background_color(&self) -> Hsla {
        self.background_color
    }

    fn has_border(&self) -> bool {
        self.has_border
    }

    fn border_color(&self) -> Hsla {
        self.border_color
    }
}
