use gpui::{prelude::FluentBuilder, *};
use std::rc::Rc;

#[derive(IntoElement)]
pub struct TextButton {
    main: Stateful<Div>,
    text: String,
    text_color: Option<Rgba>,
    text_size: Option<Pixels>,
    bg_color: Option<Rgba>,
    width: Option<Pixels>,
    height: Option<Pixels>,
    rounded: Option<Pixels>,
    on_click: Option<Rc<dyn Fn(&ClickEvent, &mut Window, &mut App)>>,
}

impl TextButton {
    pub fn new(id: impl Into<ElementId>, text: impl Into<String>) -> Self {
        Self {
            main: div().id(id.into()),
            text: text.into(),
            text_color: None,
            text_size: None,
            bg_color: None,
            width: None,
            height: None,
            rounded: None,
            on_click: None,
        }
    }

    pub fn text_color(mut self, color: impl Into<Rgba>) -> Self {
        self.text_color = Some(color.into());
        self
    }

    pub fn text_size(mut self, size: Pixels) -> Self {
        self.text_size = Some(size);
        self
    }

    pub fn bg_color(mut self, color: impl Into<Rgba>) -> Self {
        self.bg_color = Some(color.into());
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

    pub fn rounded(mut self, r: Pixels) -> Self {
        self.rounded = Some(r);
        self
    }

    pub fn on_click(
        mut self,
        callback: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Rc::new(callback));
        self
    }
}

impl RenderOnce for TextButton {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let mut div = self
            .main
            .flex()
            .items_center()
            .justify_center()
            .when_some(self.text_size, |this, size| this.text_size(size))
            .when_none(&self.text_size, |this| this.text_size(px(18.0)))
            .when_some(self.width, |this, w| this.w(w))
            .when_some(self.height, |this, h| this.h(h))
            .when_some(self.rounded, |this, r| this.rounded(r))
            .when_some(self.bg_color, |this, bg| this.bg(bg));

        if let Some(color) = self.text_color {
            div = div.text_color(color);
        }

        if let Some(on_click) = self.on_click {
            div = div.on_click(move |event, window, cx| (on_click)(event, window, cx));
        }

        div.child(self.text)
    }
}
