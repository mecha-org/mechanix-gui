use iced_native::layout::{self, Layout};
use iced_native::renderer;
use iced_native::widget::{self, Widget};
use iced_native::{Color, Element, Length, Point, Rectangle, Size};
pub enum CustomWidgetState {
    Enabled,
    Disabled,
}
pub struct CustomWidget {
    state: CustomWidgetState,
}

impl CustomWidget {
    pub fn new(state: CustomWidgetState) -> Self {
        Self { state }
    }
}

pub fn custom_widget(state: CustomWidgetState) -> CustomWidget {
    CustomWidget::new(state)
}

impl<Message, Renderer> Widget<Message, Renderer> for CustomWidget
where
    Renderer: renderer::Renderer,
{
    fn width(&self) -> Length {
        Length::Shrink
    }

    fn height(&self) -> Length {
        Length::Shrink
    }

    fn layout(&self, _renderer: &Renderer, _limits: &layout::Limits) -> layout::Node {
        layout::Node::new(Size::new(120.0, 120.0))
    }

    fn draw(
        &self,
        _state: &widget::Tree,
        renderer: &mut Renderer,
        _theme: &Renderer::Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        _cursor_position: Point,
        _viewport: &Rectangle,
    ) {
        renderer.fill_quad(
            renderer::Quad {
                bounds: layout.bounds(),
                border_radius: (120.0).into(),
                border_width: 4.0,
                border_color: Color::WHITE,
            },
            Color::TRANSPARENT,
        );
    }
}

impl<'a, Message, Renderer> From<CustomWidget> for Element<'a, Message, Renderer>
where
    Renderer: renderer::Renderer,
{
    fn from(custom_widget: CustomWidget) -> Self {
        Self::new(custom_widget)
    }
}
