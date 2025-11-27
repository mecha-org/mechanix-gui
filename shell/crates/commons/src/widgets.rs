use gpui::{
    Bounds, GlobalElementId, Hitbox, InspectorElementId, Interactivity, Pixels, Point, Size,
    StyleRefinement, Window, polygon, prelude::*, px, rgba,
};

pub struct Wing {
    interactivity: Interactivity,
    border_width: Pixels,
    upper_wing_size: Size<Pixels>,
    lower_wing_size: Size<Pixels>,
}

pub fn wing() -> Wing {
    Wing {
        interactivity: Interactivity::new(),
        border_width: px(0.0),
        upper_wing_size: Size::new(px(0.0), px(0.0)),
        lower_wing_size: Size::new(px(0.0), px(0.0)),
    }
}

impl Wing {
    pub fn border_width(&mut self, width: impl Into<Pixels>) {
        self.border_width = width.into();
    }

    pub fn upper_wing_size(&mut self, size: impl Into<Size<Pixels>>) {
        self.upper_wing_size = size.into();
    }

    pub fn lower_wing_size(&mut self, size: impl Into<Size<Pixels>>) {
        self.lower_wing_size = size.into();
    }
}

impl Element for Wing {
    type RequestLayoutState = ();
    type PrepaintState = Option<Hitbox>;

    fn id(&self) -> Option<gpui::ElementId> {
        self.interactivity.element_id.clone()
    }

    fn source_location(&self) -> Option<&'static std::panic::Location<'static>> {
        self.interactivity.source_location()
    }

    fn request_layout(
        &mut self,
        global_id: Option<&GlobalElementId>,
        inspector_id: Option<&InspectorElementId>,
        window: &mut gpui::Window,
        cx: &mut gpui::App,
    ) -> (gpui::LayoutId, Self::RequestLayoutState) {
        let layout_id = self.interactivity.request_layout(
            global_id,
            inspector_id,
            window,
            cx,
            |style, window, cx| window.request_layout(style, None, cx),
        );
        (layout_id, ())
    }

    fn prepaint(
        &mut self,
        global_id: Option<&GlobalElementId>,
        inspector_id: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        window: &mut Window,
        cx: &mut gpui::App,
    ) -> Option<Hitbox> {
        self.interactivity.prepaint(
            global_id,
            inspector_id,
            bounds,
            bounds.size,
            window,
            cx,
            |_, _, hitbox, _, _| hitbox,
        )
    }

    fn paint(
        &mut self,
        _global_id: Option<&gpui::GlobalElementId>,
        _inspector_id: Option<&gpui::InspectorElementId>,
        bounds: gpui::Bounds<gpui::Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        _hitbox: &mut Self::PrepaintState,
        window: &mut gpui::Window,
        _cx: &mut gpui::App,
    ) {
        // self.interactivity.paint(
        //     global_id,
        //     inspector_id,
        //     bounds,
        //     hitbox.as_ref(),
        //     window,
        //     cx,
        //     |_style, window, _cx| {},
        // );

        let width = bounds.size.width;
        let height = bounds.size.height;

        let upper_wing_width = self.upper_wing_size.width;
        let upper_wing_height = self.upper_wing_size.height;
        let lower_wing_width = self.lower_wing_size.width;
        let lower_wing_height = self.lower_wing_size.height;

        let upper_corners = if upper_wing_height > px(0.1) && upper_wing_width > px(0.1) {
            vec![
                bounds.origin
                    + Point::new(
                         px(0.0),
                        -upper_wing_height,
                    ),
                bounds.origin + Point::new(upper_wing_width, -upper_wing_height),
                bounds.origin + Point::new(upper_wing_width + upper_wing_height, px(0.0)),
                bounds.origin + Point::new(width, px(0.0)),
            ]
        } else {
            vec![
                bounds.origin,
                bounds.origin + Point::new(width, px(0.0)),
            ]
        };

        let lower_corners = if lower_wing_height > px(0.1) && lower_wing_width > px(0.1) {
            vec![
                bounds.origin + Point::new(width, height),
                bounds.origin + Point::new(width - lower_wing_width, height),
                bounds.origin
                    + Point::new(
                        width - lower_wing_width - lower_wing_height,
                        height - lower_wing_height,
                    ),
                bounds.origin + Point::new(px(0.0), height - lower_wing_height),
            ]
        } else {
            vec![
                bounds.origin + Point::new(width, height),
                bounds.origin + Point::new(px(0.0), height),
            ]
        };

        let mut points = vec![];
        points.extend(upper_corners);
        points.extend(lower_corners);
        points.dedup();

        let background = if let Some(fill) = &self.interactivity.base_style.background {
            if let Some(color) = fill.color() {
                color
            } else {
                rgba(0).into()
            }
        } else {
            rgba(0).into()
        };
        let polygon = polygon(points, background);

        let border_color = if let Some(color) = self.interactivity.base_style.border_color {
                color
        } else {
            rgba(0).into()
        };
        let polygon = polygon.border_color(border_color);
        let polygon = polygon.border_width(self.border_width);
        window.paint_polygon(polygon);
    }
}

impl IntoElement for Wing {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

impl Styled for Wing {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.interactivity.base_style
    }
}

impl InteractiveElement for Wing {
    fn interactivity(&mut self) -> &mut Interactivity {
        &mut self.interactivity
    }
}

