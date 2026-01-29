use dispatcher::Dispatcher;
use gpui::*;
use icons::prelude::*;
use mxsearch::prelude::AppInfo;
use settings::prelude::Settings;
use theme::prelude::{AlphaExt, Fonts, Theme};

pub mod bluetooth_modal;
pub mod display_modal;
pub mod extended_screen;
pub mod performance_modal;
pub mod sound_modal;
pub mod wireless_modal;

use crate::ui::{FINAL_MODAL_SIZE, ModalKind, SettingsDrawer};

pub const ROW_HEIGHT: f32 = 60.0;
pub const MODAL_HEADER_HEIGHT: f32 = 60.0;
pub const MODAL_WING_WIDTH: f32 = 237.0;
pub const MODAL_WING_HEIGHT: f32 = 36.0;

impl SettingsDrawer {
    pub fn render_header_div(
        &self,
        cx: &mut gpui::Context<SettingsDrawer>,
        title: &'static str,
    ) -> AnyElement {
        let colors = Theme::global(cx).colors.clone();

        div()
            .child(title)
            .flex()
            .w_full()
            .justify_start()
            .text_color(colors.foreground_200)
            .text_size(if self.modal_size != FINAL_MODAL_SIZE {
                px(20.)
            } else {
                px(26.)
            })
            .font_weight(FontWeight::SEMIBOLD)
            .pl(px(16.))
            .pt(px(8.))
            .h(px(MODAL_HEADER_HEIGHT))
            .into_any()
    }

    pub fn launch_app_with_path(
        &mut self,
        app_info: Option<AppInfo>,
        cx: &mut Context<Self>,
        settings_path: String,
    ) {
        if app_info.is_none() {
            return;
        }
        let sender = Dispatcher::global(cx).0.clone();
        let app_info = app_info.clone().unwrap();

        let exec = format!(
            "{}={} {}",
            "MECHNIX_SETTINGS_OPEN_PATH", settings_path, app_info.exec
        );

        println!("OPEN exec : {exec:?}");
        cx.background_executor()
            .spawn(async move {
                _ = sender
                    .broadcast(dispatcher::Message::LaunchApp {
                        app_id: app_info.possible_app_id,
                        exec: exec,
                    })
                    .await;
            })
            .detach();

        let settings = Settings::global(cx).settings_drawer.clone();
        let closed_pos: f32 = Self::calculate_closed_position(&settings);
        self.is_visible = false;
        self.position = 100.;
        self.snap_to(closed_pos, cx);
    }

    pub fn render_settings_div(
        &self,
        cx: &mut gpui::Context<SettingsDrawer>,
        settings_path: String,
    ) -> AnyElement {
        let colors = Theme::global(cx).colors.clone();
        let settings = Icons::global(cx).settings_drawer.clone().settings;

        div()
            .id("id_settings")
            .flex()
            .flex_row()
            .items_end()
            .justify_start()
            .items_center()
            .bg(colors.background_1000)
            .border_t_1()
            .border_color(colors.background_800)
            .h(px(ROW_HEIGHT))
            .rounded_md()
            .text_size(if self.modal_size != FINAL_MODAL_SIZE {
                px(18.)
            } else {
                px(22.)
            })
            .p_4()
            .flex_shrink_0()
            .child(
                svg()
                    .external_path(SharedString::from(settings.to_string_lossy().to_string()))
                    .size(px(30.))
                    .text_color(colors.accent_300),
            )
            .child(
                div()
                    .pl_2()
                    .font_weight(FontWeight::NORMAL)
                    .text_color(colors.accent_300)
                    .child("Settings"),
            )
            .on_click(
                cx.listener(Self::click_listener(move |this, _event, _window, cx| {
                    this.launch_app_with_path(
                        this.settings_app_info.clone(),
                        cx,
                        settings_path.clone(),
                    );
                    cx.notify();
                })),
            )
            .into_any()
    }

    pub fn get_icon_and_text_color(
        is_active: bool,
        cx: &mut gpui::Context<SettingsDrawer>,
    ) -> (Rgba, Rgba) {
        let colors = Theme::global(cx).colors.clone();

        let icon_color = if is_active {
            colors.accent_200
        } else {
            colors.foreground_900
        };

        let text_color = if is_active {
            colors.accent_200
        } else {
            colors.foreground_300
        };
        (icon_color, text_color)
    }
}
