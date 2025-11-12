use std::rc::Rc;

use crate::ui::icon::Icon;
use gpui::{prelude::FluentBuilder, *};

use std::sync::LazyLock;

pub static ICON_COLOR: LazyLock<Rgba> = LazyLock::new(|| rgb(0x4D4D4D)); // default - gray | passed-white or active - blue
pub static ACTIVE_ICON_COLOR: LazyLock<Rgba> = LazyLock::new(|| rgb(0x4892F1));
pub static ACTIVE_BG_COLOR: LazyLock<Rgba> = LazyLock::new(|| rgb(0x202020));
pub static PRESSED_BG_COLOR: LazyLock<Rgba> = LazyLock::new(|| rgb(0x363636)); // default - light gray | passed - yellow - xDB9200

#[derive(IntoElement)]
pub struct IconButton {
    main: Stateful<Div>,
    icon: Option<Icon>,
    disabled: bool,
    pressed: bool,
    active: bool,
    on_click: Option<Rc<dyn Fn(&ClickEvent, &mut Window, &mut App)>>,
    size: Option<(Pixels, Pixels)>,
    icon_color: Option<Hsla>,
    active_icon_color: Option<Hsla>,
    active_bg_color: Option<Hsla>,
    border: Option<Pixels>,
    label: Option<String>,
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
            active_icon_color: None,
            active_bg_color: None,
            border: None,
            label: None,
        }
    }

    pub fn icon(mut self, icon: impl Into<Icon>) -> Self {
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

    pub fn size(mut self, size: impl Into<(Pixels, Pixels)>) -> Self {
        let (width, height) = size.into();
        self.size = Some((width, height));
        self
    }

    pub fn icon_color(mut self, icon_color: impl Into<Hsla>) -> Self {
        self.icon_color = Some(icon_color.into());
        self
    }

    pub fn active_icon_color(mut self, active_icon_color: impl Into<Hsla>) -> Self {
        self.active_icon_color = Some(active_icon_color.into());
        self
    }

    pub fn active_bg_color(mut self, active_bg_color: impl Into<Hsla>) -> Self {
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
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let main = self
            .main
            .flex()
            .w(px(84.0))
            .h(px(84.0))
            .rounded(px(8.0))
            .border(px(1.))
            .border_color(rgb(0x4D4D4D))
            .active(|this| this.opacity(0.85))
            // .opacity(0.2)
            .items_center()
            .justify_center()
            .when(self.pressed, |this| this.bg(PRESSED_BG_COLOR.to_owned())) // BG - light gray - pressed
            .when(!self.pressed && self.active, |this| {
                if let Some(active_bg_color) = self.active_bg_color {
                    this.bg(active_bg_color)
                } else {
                    this.bg(ACTIVE_BG_COLOR.to_owned())
                }
               
            }) // BG - dark gray - active
            .when_some(self.on_click, |this, on_click| {
                this.on_click(move |event, window, cx| (on_click)(event, window, cx))
            })
            // .when_some(self.icon, |this, icon| {
            //     if let Some(icon_color) = self.icon_color {
            //         this.child(icon.text_color(icon_color))       // passing gray for default
            //     } else {
            //         this.child(icon.text_color(rgb(0x4892F1)))   // blue - icon color
            //     }
            // })
            .when_some(self.icon, |this, icon| {
                let color: Hsla = if self.active {
                    if let Some(active_icon_color) = self.active_icon_color {
                        active_icon_color
                    } else {
                        ACTIVE_ICON_COLOR.to_owned().into()
                    }
                } else {
                    if let Some(icon_color) = self.icon_color {
                        icon_color
                    } else {
                        ICON_COLOR.to_owned().into()
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
                .text_sm()
                .text_color(rgb(0xF4F4F4))
        } else {
            main
        }
    }
}
