use gpui::{
    AnyElement, Bounds, GlobalElementId, Hitbox, InspectorElementId, Interactivity, LayoutId,
    Pixels, Point, Size, StyleRefinement, Window, polygon, prelude::*, px, rgba,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WingSide {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CornerRadii {
    pub top_left: Pixels,
    pub top_right: Pixels,
    pub bottom_right: Pixels,
    pub bottom_left: Pixels,
}

impl CornerRadii {
    pub fn all(radius: Pixels) -> Self {
        Self {
            top_left: radius,
            top_right: radius,
            bottom_right: radius,
            bottom_left: radius,
        }
    }

    pub fn none() -> Self {
        Self::all(px(0.0))
    }
}

fn generate_fillet_arc(
    prev: Point<Pixels>,
    current: Point<Pixels>,
    next: Point<Pixels>,
    radius: Pixels,
    resolution: u32,
) -> Vec<Point<Pixels>> {
    let v1 = Point::new(prev.x - current.x, prev.y - current.y);
    let v2 = Point::new(next.x - current.x, next.y - current.y);

    let v1_x = v1.x.to_f64() as f32;
    let v1_y = v1.y.to_f64() as f32;
    let v2_x = v2.x.to_f64() as f32;
    let v2_y = v2.y.to_f64() as f32;

    let len1 = (v1_x * v1_x + v1_y * v1_y).sqrt();
    let len2 = (v2_x * v2_x + v2_y * v2_y).sqrt();

    if len1 < 0.001 || len2 < 0.001 {
        return vec![current];
    }

    let n1_x = v1_x / len1;
    let n1_y = v1_y / len1;
    let n2_x = v2_x / len2;
    let n2_y = v2_y / len2;

    let dot = n1_x * n2_x + n1_y * n2_y;
    let angle = dot.clamp(-1.0, 1.0).acos();

    if !(0.01..=std::f32::consts::PI - 0.01).contains(&angle) {
        return vec![current];
    }

    let half_angle = angle / 2.0;
    let radius_f32 = radius.to_f64() as f32;
    let offset_distance = radius_f32 / half_angle.tan();

    let max_offset = (len1.min(len2) * 0.5).min(offset_distance);
    let offset_distance = max_offset;

    let effective_radius = offset_distance * half_angle.tan();

    let start_point = Point::new(
        current.x + px(n1_x * offset_distance),
        current.y + px(n1_y * offset_distance),
    );
    let end_point = Point::new(
        current.x + px(n2_x * offset_distance),
        current.y + px(n2_y * offset_distance),
    );

    let bisector_x = n1_x + n2_x;
    let bisector_y = n1_y + n2_y;
    let bisector_len = (bisector_x * bisector_x + bisector_y * bisector_y).sqrt();

    if bisector_len < 0.001 {
        return vec![current];
    }

    let bisector_norm_x = bisector_x / bisector_len;
    let bisector_norm_y = bisector_y / bisector_len;

    let center_distance = effective_radius / half_angle.sin();
    let center = Point::new(
        current.x + px(bisector_norm_x * center_distance),
        current.y + px(bisector_norm_y * center_distance),
    );

    let mut arc_points = Vec::new();

    let start_angle = (start_point.y - center.y)
        .to_f64()
        .atan2((start_point.x - center.x).to_f64()) as f32;
    let end_angle = (end_point.y - center.y)
        .to_f64()
        .atan2((end_point.x - center.x).to_f64()) as f32;

    let mut angle_diff = end_angle - start_angle;
    if angle_diff > std::f32::consts::PI {
        angle_diff -= 2.0 * std::f32::consts::PI;
    } else if angle_diff < -std::f32::consts::PI {
        angle_diff += 2.0 * std::f32::consts::PI;
    }

    for i in 0..=resolution {
        let t = i as f32 / resolution as f32;
        let current_angle = start_angle + angle_diff * t;

        let point = Point::new(
            center.x + px(effective_radius * current_angle.cos()),
            center.y + px(effective_radius * current_angle.sin()),
        );
        arc_points.push(point);
    }

    arc_points
}

fn apply_fillet_to_polygon_with_radii(
    points: &[Point<Pixels>],
    radii: &[Pixels],
    resolution: u32,
) -> Vec<Point<Pixels>> {
    if points.len() < 3 {
        return points.to_vec();
    }

    let mut filleted_points = Vec::new();
    let n = points.len();

    for i in 0..n {
        let prev = points[(i + n - 1) % n];
        let current = points[i];
        let next = points[(i + 1) % n];
        let radius = radii[i.min(radii.len() - 1)];

        let arc = generate_fillet_arc(prev, current, next, radius, resolution);
        filleted_points.extend(arc);
    }

    filleted_points
}

pub struct Wing {
    interactivity: Interactivity,
    border_width: Pixels,
    corner_radii: CornerRadii,
    border_resolution: u32,
    upper_wing_size: Size<Pixels>,
    lower_wing_size: Size<Pixels>,
    upper_wing_side: WingSide,
    lower_wing_side: WingSide,
    include_upper_wing_in_bounds: bool,
    include_lower_wing_in_bounds: bool,
    children: Vec<AnyElement>,
}

pub fn wing() -> Wing {
    Wing {
        interactivity: Interactivity::new(),
        border_width: px(0.0),
        corner_radii: CornerRadii::none(),
        border_resolution: 8,
        upper_wing_size: Size::new(px(0.0), px(0.0)),
        lower_wing_size: Size::new(px(0.0), px(0.0)),
        upper_wing_side: WingSide::Left,
        lower_wing_side: WingSide::Right,
        include_upper_wing_in_bounds: true,
        include_lower_wing_in_bounds: true,
        children: Vec::new(),
    }
}

impl Wing {
    pub fn border_width(&mut self, width: impl Into<Pixels>) {
        self.border_width = width.into();
    }

    // Set all corners to the same radius
    pub fn border_radius(&mut self, radius: impl Into<Pixels>) {
        let r = radius.into();
        self.corner_radii = CornerRadii::all(r);
    }

    // Set individual corner radii
    pub fn corner_radii(&mut self, radii: CornerRadii) {
        self.corner_radii = radii;
    }

    pub fn border_resolution(&mut self, resolution: impl Into<u32>) {
        self.border_resolution = resolution.into();
    }

    pub fn upper_wing_size(&mut self, size: impl Into<Size<Pixels>>) {
        self.upper_wing_size = size.into();
    }

    pub fn lower_wing_size(&mut self, size: impl Into<Size<Pixels>>) {
        self.lower_wing_size = size.into();
    }

    pub fn upper_wing_side(&mut self, side: WingSide) {
        self.upper_wing_side = side;
    }

    pub fn lower_wing_side(&mut self, side: WingSide) {
        self.lower_wing_side = side;
    }

    pub fn include_upper_wing_in_bounds(&mut self, include: bool) {
        self.include_upper_wing_in_bounds = include;
    }

    pub fn include_lower_wing_in_bounds(&mut self, include: bool) {
        self.include_lower_wing_in_bounds = include;
    }

    // Helper to map corner radii to polygon points
    fn get_radii_for_points(&self, points: &[Point<Pixels>], bounds: Bounds<Pixels>) -> Vec<Pixels> {
        let width = bounds.size.width;
        let height = bounds.size.height;
        let origin = bounds.origin;

        points.iter().map(|point| {
            let rel_x = point.x - origin.x;
            let rel_y = point.y - origin.y;

            // Determine which corner this point is closest to
            let is_top = rel_y < height / 2.0;
            let is_left = rel_x < width / 2.0;

            match (is_top, is_left) {
                (true, true) => self.corner_radii.top_left,
                (true, false) => self.corner_radii.top_right,
                (false, false) => self.corner_radii.bottom_right,
                (false, true) => self.corner_radii.bottom_left,
            }
        }).collect()
    }
}

impl Element for Wing {
    type RequestLayoutState = Vec<LayoutId>;
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
        let mut child_layout_ids = Vec::new();
        for child in &mut self.children {
            child_layout_ids.push(child.request_layout(window, cx));
        }

        let layout_id = self.interactivity.request_layout(
            global_id,
            inspector_id,
            window,
            cx,
            |style, window, cx| window.request_layout(style, child_layout_ids.iter().copied(), cx),
        );
        (layout_id, child_layout_ids)
    }

    fn prepaint(
        &mut self,
        global_id: Option<&GlobalElementId>,
        inspector_id: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        request_layout: &mut Self::RequestLayoutState,
        window: &mut Window,
        cx: &mut gpui::App,
    ) -> Option<Hitbox> {
        for child in &mut self.children {
            child.prepaint(window, cx);
        }

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
        cx: &mut gpui::App,
    ) {
        let width = bounds.size.width;
        let height = bounds.size.height;

        let upper_wing_width = self.upper_wing_size.width;
        let upper_wing_height = self.upper_wing_size.height;
        let lower_wing_width = self.lower_wing_size.width;
        let lower_wing_height = self.lower_wing_size.height;

        // Generate upper wing corners based on side
        let upper_corners = if upper_wing_height > px(0.1) && upper_wing_width > px(0.1) {
            match self.upper_wing_side {
                WingSide::Left => {
                    if self.include_upper_wing_in_bounds {
                        vec![
                            bounds.origin + Point::new(px(0.0), px(0.0)),
                            bounds.origin + Point::new(upper_wing_width, px(0.0)),
                            bounds.origin
                                + Point::new(upper_wing_width + upper_wing_height, upper_wing_height),
                            bounds.origin + Point::new(width, upper_wing_height),
                        ]
                    } else {
                        vec![
                            bounds.origin + Point::new(px(0.0), -upper_wing_height),
                            bounds.origin + Point::new(upper_wing_width, -upper_wing_height),
                            bounds.origin + Point::new(upper_wing_width + upper_wing_height, px(0.0)),
                            bounds.origin + Point::new(width, px(0.0)),
                        ]
                    }
                }
                WingSide::Right => {
                    if self.include_upper_wing_in_bounds {
                        vec![
                            bounds.origin + Point::new(px(0.0), upper_wing_height),
                            bounds.origin
                                + Point::new(width - upper_wing_width - upper_wing_height, upper_wing_height),
                            bounds.origin + Point::new(width - upper_wing_width, px(0.0)),
                            bounds.origin + Point::new(width, px(0.0)),
                        ]
                    } else {
                        vec![
                            bounds.origin + Point::new(px(0.0), px(0.0)),
                            bounds.origin
                                + Point::new(width - upper_wing_width - upper_wing_height, px(0.0)),
                            bounds.origin + Point::new(width - upper_wing_width, -upper_wing_height),
                            bounds.origin + Point::new(width, -upper_wing_height),
                        ]
                    }
                }
            }
        } else {
            vec![bounds.origin, bounds.origin + Point::new(width, px(0.0))]
        };

        // Generate lower wing corners based on side
        let lower_corners = if lower_wing_height > px(0.1) && lower_wing_width > px(0.1) {
            match self.lower_wing_side {
                WingSide::Right => {
                    if self.include_lower_wing_in_bounds {
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
                            bounds.origin + Point::new(width, height + lower_wing_height),
                            bounds.origin
                                + Point::new(width - lower_wing_width, height + lower_wing_height),
                            bounds.origin
                                + Point::new(width - lower_wing_width - lower_wing_height, height),
                            bounds.origin + Point::new(px(0.0), height),
                        ]
                    }
                }
                WingSide::Left => {
                    if self.include_lower_wing_in_bounds {
                        vec![
                            bounds.origin + Point::new(width, height - lower_wing_height),
                            bounds.origin
                                + Point::new(lower_wing_width + lower_wing_height, height - lower_wing_height),
                            bounds.origin + Point::new(lower_wing_width, height),
                            bounds.origin + Point::new(px(0.0), height),
                        ]
                    } else {
                        vec![
                            bounds.origin + Point::new(width, height),
                            bounds.origin
                                + Point::new(lower_wing_width + lower_wing_height, height),
                            bounds.origin + Point::new(lower_wing_width, height + lower_wing_height),
                            bounds.origin + Point::new(px(0.0), height + lower_wing_height),
                        ]
                    }
                }
            }
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

        // Apply individual corner radii if any radius is > 0
        let has_radius = self.corner_radii.top_left > px(0.0)
            || self.corner_radii.top_right > px(0.0)
            || self.corner_radii.bottom_right > px(0.0)
            || self.corner_radii.bottom_left > px(0.0);

        let filleted_points = if has_radius && points.len() >= 3 {
            let radii = self.get_radii_for_points(&points, bounds);
            apply_fillet_to_polygon_with_radii(&points, &radii, self.border_resolution)
        } else {
            points
        };

        let background = if let Some(fill) = &self.interactivity.base_style.background {
            if let Some(color) = fill.color() {
                color
            } else {
                rgba(0).into()
            }
        } else {
            rgba(0).into()
        };
        let polygon = polygon(filleted_points, background);

        let border_color = if let Some(color) = self.interactivity.base_style.border_color {
            color
        } else {
            rgba(0).into()
        };
        let polygon = polygon.border_color(border_color);
        let polygon = polygon.border_width(self.border_width);
        window.paint_polygon(polygon);

        // Paint all children
        for child in &mut self.children {
            child.paint(window, cx);
        }
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

impl ParentElement for Wing {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}