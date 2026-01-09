// icon_button.rs
use std::rc::Rc;

use crate::ui::icon::Icon;
use gpui::{ prelude::FluentBuilder, * };
use theme::prelude::Theme;

#[derive(IntoElement)]
pub struct IconButton {
    main: Stateful<Div>,
    icon: Option<Icon>,
    disabled: bool,
    pressed: bool,
    active: bool,
    on_click: Option<Rc<dyn Fn(&ClickEvent, &mut Window, &mut App)>>,
    icon_color: Option<Hsla>,
    width: Option<Pixels>,
    height: Option<Pixels>,
    boxSize: Option<Pixels>,
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
            icon_color: None,
            width: None,
            height: None,
            boxSize: None,
        }
    }

    pub fn icon(mut self, icon: impl Into<Icon>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    pub fn on_click(
        mut self,
        callback: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static
    ) -> Self {
        self.on_click = Some(Rc::new(callback));
        self
    }

    pub fn icon_color(mut self, icon_color: impl Into<Hsla>) -> Self {
        self.icon_color = Some(icon_color.into());
        self
    }

    pub fn width(mut self, w: Pixels) -> Self {
        self.width = Some(w);
        self
    }

    pub fn height(mut self, h: Pixels) -> Self {
        self.height = Some(h);
        self
    }

    pub fn size(mut self, s: Pixels) -> Self {
        self.width = Some(s);
        self.height = Some(s);
        self
    }
}

impl RenderOnce for IconButton {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let colors = Theme::global(_cx).colors.clone();

        div()
            .bg(colors.background_800)
            .flex()
            .items_center()
            .justify_center()
            .rounded(px(5.6))
            .when_some(self.boxSize, |this, s| this.w(s).h(s))
            .when_none(&self.boxSize, |this| this.size(px(56.0)))
            .child(
                self.main
                    .flex()
                    .when_some(self.width, |this, w| this.w(w))
                    .when_none(&self.width, |this| this.w(px(41.0)))
                    .when_some(self.height, |this, h| this.h(h))
                    .when_none(&self.height, |this| this.h(px(41.0)))
                    .border(px(1.0))
                    .items_center()
                    .justify_center()
                    // .when(self.pressed, |this| this.bg(rgb(0x363636)))
                    // .when(!self.pressed && self.active, |this| this.bg(rgb(0x202020)))
                    .when_some(self.on_click, |this, on_click| {
                        this.on_click(move |event, window, cx| on_click(event, window, cx))
                    })
                    .when_some(self.icon, |this, icon| {
                        if let Some(icon_color) = self.icon_color {
                            this.child(div().size(px(41.0)))
                            // .child(icon.text_color(icon_color)))
                        } else {
                            this.child(icon)
                        }
                    })
            )
    }
}
