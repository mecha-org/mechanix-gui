use std::rc::Rc;

use gpui::{LongPressEvent, prelude::FluentBuilder, *};
use theme::prelude::{AlphaExt, Fonts, Theme};

#[derive(IntoElement)]
pub struct IconButton {
    main: Stateful<Div>,
    icon: Option<Svg>,
    disabled: bool,
    pressed: bool,
    active: bool,
    on_click: Option<Rc<dyn Fn(&ClickEvent, &mut Window, &mut App)>>,
    size: Option<(Pixels, Pixels)>,
    icon_color: Option<Rgba>,
    bg_color: Option<Rgba>,
    active_icon_color: Option<Rgba>,
    active_bg_color: Option<Rgba>,
    border: Option<Pixels>,
    label: Option<String>,
    on_long_press: Option<Rc<dyn Fn(&LongPressEvent, &mut Window, &mut App)>>,
    long_press_duration_ms: Option<u64>,
}

impl IconButton {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            main: div().id(id.into()),
            icon: None,
            disabled: false,
            pressed: false,
            active: false,
            on_click: None,
            size: None,
            icon_color: None,
            bg_color: None,
            active_icon_color: None,
            active_bg_color: None,
            border: None,
            label: None,
            on_long_press: None,
            long_press_duration_ms: None,
        }
    }

    pub fn icon(mut self, icon: impl Into<Svg>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    pub fn on_click(
        mut self,
        callback: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Rc::new(callback));
        self
    }

    // default duration (500ms)
    pub fn on_long_press(
        mut self,
        callback: impl Fn(&LongPressEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_long_press = Some(Rc::new(callback));
        self
    }

    pub fn on_long_press_ms(
        mut self,
        duration_ms: u64,
        callback: impl Fn(&LongPressEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.long_press_duration_ms = Some(duration_ms);
        self.on_long_press = Some(Rc::new(callback));
        self
    }

    pub fn size(mut self, size: impl Into<(Pixels, Pixels)>) -> Self {
        let (width, height) = size.into();
        self.size = Some((width, height));
        self
    }

    pub fn icon_color(mut self, icon_color: impl Into<Rgba>) -> Self {
        self.icon_color = Some(icon_color.into());
        self
    }

    pub fn bg_color(mut self, bg_color: impl Into<Rgba>) -> Self {
        self.bg_color = Some(bg_color.into());
        self
    }

    pub fn active_icon_color(mut self, active_icon_color: impl Into<Rgba>) -> Self {
        self.active_icon_color = Some(active_icon_color.into());
        self
    }

    pub fn active_bg_color(mut self, active_bg_color: impl Into<Rgba>) -> Self {
        self.active_bg_color = Some(active_bg_color.into());
        self
    }

    pub fn border(mut self, border: impl Into<Pixels>) -> Self {
        self.border = Some(border.into());
        self
    }

    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn active(mut self, active: bool) -> Self {
        self.active = active;
        self
    }
}

impl RenderOnce for IconButton {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let colors = Theme::global(cx).colors.clone();
        let primary_font = Fonts::global(cx).primary.clone();

        let t_icon_color = colors.foreground_400;
        let t_active_icon_color = colors.accent_200; // default - gray | custom can be - amber
        let t_active_icon_bg_color = colors.accent_200.with_alpha(0.1); // for pressed 
        let t_long_press_active_icon_bg_color = colors.accent_200.with_alpha(0.2);
        let t_bg_color = colors.background_900;
        let t_border_color = colors.accent_400.with_alpha(0.4);

        let main = self
            .main
            .flex()
            .w(px(84.0))
            .h(px(84.0))
            .rounded(px(8.0))
            .border(if self.active { px(1.5) } else { px(0.) })
            .border_color(t_border_color) // KEEP THIS
            .active(|this| {
                let mut style = this.clone();
                style = style
                    .clone()
                    .bg(if self.on_long_press.is_some() {
                        t_long_press_active_icon_bg_color
                    } else if let Some(active_bg_color) = self.active_bg_color {
                        active_bg_color
                    } else {
                        t_active_icon_bg_color
                    })
                    .into();

                style = style.border(px(1.5)).border_color(t_border_color);
                style
            }) // GPUI's active state
            .items_center()
            .justify_center()
            .when(!self.pressed && !self.active, |this| {
                this.bg(if let Some(bg_color) = self.bg_color {
                    bg_color
                } else {
                    t_bg_color
                })
            })
            .when(!self.pressed && self.active, |this| {
                if let Some(active_bg_color) = self.active_bg_color {
                    this.bg(active_bg_color)
                } else {
                    this
                }
            })
            .when_some(self.on_click, |this, on_click| {
                this.on_click(move |event, window, cx| (on_click)(event, window, cx))
            })
            .when_some(self.on_long_press.clone(), |this, on_long_press| {
                if let Some(duration_ms) = self.long_press_duration_ms {
                    this.on_long_press_ms(duration_ms, move |event, window, cx| {
                        (on_long_press)(event, window, cx)
                    })
                } else {
                    this.on_long_press(move |event, window, cx| (on_long_press)(event, window, cx))
                }
            })
            .when_some(self.icon, |this, icon| {
                let color: Rgba = if self.active {
                    if let Some(active_icon_color) = self.active_icon_color {
                        active_icon_color
                    } else {
                        t_active_icon_color.into()
                    }
                } else {
                    if let Some(icon_color) = self.icon_color {
                        icon_color
                    } else {
                        t_icon_color.into()
                    }
                };

                this.child(icon.text_color(color))
            })
            .when_some(self.size, |this, sz| this.w(sz.0).h(sz.1))
            .when_some(self.border, |this, border| this.border(border));

        if let Some(label) = self.label {
            main.flex_col()
                .justify_center()
                .child(label)
                .text_size(px(12.))
                .font_weight(FontWeight::NORMAL)
                .font_family(primary_font)
                .text_color(colors.foreground_600)
        } else {
            main
        }
    }
}
