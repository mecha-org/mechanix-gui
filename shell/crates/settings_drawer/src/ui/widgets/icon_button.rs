use std::rc::Rc;

use crate::{constants::*, prelude::*, ui::icon::Icon};
use gpui::{prelude::FluentBuilder, *};
use theme::prelude::Theme;

const ICON_COLOR: u32 = DARK_NEUTRAL_100; // default - gray | custom can be - white or active - amber
const ACTIVE_ICON_COLOR: u32 = AMBER_600; // default - gray | custom can be - amber
const ACTIVE_ICON_BG_COLOR: u32 = 0xC6760040; // for pressed
const BG_COLOR: u32 = DARK_NEUTRAL_900;
const BORDER_COLOR: u32 = AMBER_1000;

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
    bg_color: Option<Hsla>,
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
            bg_color: None,
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

    pub fn bg_color(mut self, bg_color: impl Into<Hsla>) -> Self {
        self.bg_color = Some(bg_color.into());
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
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let colors = Theme::global(cx).colors.clone();

        let main = self
            .main
            .flex()
            .w(px(84.0))
            .h(px(84.0))
            .rounded(px(8.0))
            .border(if self.active { px(1.5) } else { px(0.) })
            .border_color(rgb(BORDER_COLOR)) // KEEP THIS
            .active(|this| {
                // todo: update icon color
                let mut style = this.clone();
                style = style
                    .clone()
                    .bg(if let Some(active_bg_color) = self.active_bg_color {
                        active_bg_color
                    } else {
                        rgba(ACTIVE_ICON_BG_COLOR).into()
                    })
                    .border(px(1.5))
                    .border_color(rgb(BORDER_COLOR));
                style
            }) // GPUI's active state
            .items_center()
            .justify_center()
            .when(!self.pressed && !self.active, |this| {
                this.bg(if let Some(bg_color) = self.bg_color {
                    bg_color
                } else {
                    rgb(BG_COLOR).into()
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
            .when_some(self.icon, |this, icon| {
                // todo: check if this.active is true - update icon color
                // let child = this.active(|style| {
                //     style.text_color(if let Some(active_icon_color) = self.active_icon_color {
                //         active_icon_color
                //     } else {
                //         rgb(ACTIVE_ICON_COLOR).into()
                //     })
                // });

                let color: Hsla = if self.active {
                    if let Some(active_icon_color) = self.active_icon_color {
                        active_icon_color
                    } else {
                        rgb(ACTIVE_ICON_COLOR).into()
                    }
                } else {
                    if let Some(icon_color) = self.icon_color {
                        icon_color
                    } else {
                        rgb(ICON_COLOR).into()
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
                .text_color(rgb(DARK_NEUTRAL_100))
        } else {
            main
        }
    }
}
