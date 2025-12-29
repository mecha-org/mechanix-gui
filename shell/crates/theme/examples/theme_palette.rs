use gpui::{
    App, Application, Bounds, Context, Rgba, Window, WindowBounds, WindowOptions, div, prelude::*,
    px, rgb, size,
};
use theme::ActiveTheme;

struct ThemePalette {}

impl ThemePalette {
    fn color_swatch(&self, label: &str, color: Rgba) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap_1()
            .child(
                div()
                    .w(px(80.0))
                    .h(px(60.0))
                    .bg(color)
                    .border_1()
                    .border_color(rgb(0x333333))
                    .rounded_md()
                    .shadow_sm(),
            )
            .child(
                div()
                    .text_xs()
                    .text_color(rgb(0xcccccc))
                    .child(label.to_string()),
            )
            .child(div().text_xs().text_color(rgb(0xcccccc)).child(format!(
                "rgba({:?}, {:?}, {:?}, {:?})",
                (color.r * 255.) as i32,
                (color.g * 255.) as i32,
                (color.b * 255.) as i32,
                (color.a) as i32,
            )))
    }

    fn color_group(
        &self,
        title: &str,
        colors: Vec<(&str, Rgba)>,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap_3()
            .child(
                div()
                    .text_lg()
                    .font_weight(gpui::FontWeight::BOLD)
                    .text_color(rgb(0xffffff))
                    .child(title.to_string()),
            )
            .child(
                div().flex().flex_wrap().gap_4().children(
                    colors
                        .into_iter()
                        .map(|(label, color)| self.color_swatch(label, color)),
                ),
            )
    }
}

impl Render for ThemePalette {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let colors = cx.theme().colors.clone();

        let accent_colors = vec![
            ("accent 0", colors.accent_0),
            ("accent 100", colors.accent_100),
            ("accent 200", colors.accent_200),
            ("accent 300", colors.accent_300),
            ("accent 400", colors.accent_400),
            ("accent 500", colors.accent_500),
            ("accent 600", colors.accent_600),
            ("accent 700", colors.accent_700),
            ("accent 800", colors.accent_800),
            ("accent 900", colors.accent_900),
            ("accent 1000", colors.accent_1000),
        ];

        let background_colors = vec![
            ("bg 0", colors.background_0),
            ("bg 100", colors.background_100),
            ("bg 200", colors.background_200),
            ("bg 300", colors.background_300),
            ("bg 400", colors.background_400),
            ("bg 500", colors.background_500),
            ("bg 600", colors.background_600),
            ("bg 700", colors.background_700),
            ("bg 800", colors.background_800),
            ("bg 900", colors.background_900),
            ("bg 1000", colors.background_1000),
        ];

        let foreground_colors = vec![
            ("fg 0", colors.foreground_0),
            ("fg 100", colors.foreground_100),
            ("fg 200", colors.foreground_200),
            ("fg 300", colors.foreground_300),
            ("fg 400", colors.foreground_400),
            ("fg 500", colors.foreground_500),
            ("fg 600", colors.foreground_600),
            ("fg 700", colors.foreground_700),
            ("fg 800", colors.foreground_800),
            ("fg 900", colors.foreground_900),
            ("fg 1000", colors.foreground_1000),
        ];

        div()
            .flex()
            .flex_col()
            .gap_6()
            .bg(rgb(0x1a1a1a))
            .w_full()
            .h_full()
            .p_8()
            .child(
                div()
                    .text_2xl()
                    .font_weight(gpui::FontWeight::BOLD)
                    .text_color(rgb(0xffffff))
                    .child("Theme Color Palette"),
            )
            .child(self.color_group("Accent Colors", accent_colors, cx))
            .child(self.color_group("Background Colors", background_colors, cx))
            .child(self.color_group("Foreground Colors", foreground_colors, cx))
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(1200.), px(1000.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| {
                cx.new(|cx| {
                    theme::init(cx);
                    ThemePalette {}
                })
            },
        )
        .unwrap();
        cx.activate(true);
    });
}
