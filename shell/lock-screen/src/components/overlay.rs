use std::hash::Hash;

use mctk_core::{
    component::{Component, RenderContext},
    lay,
    layout::PositionType,
    node, rect,
    reexports::femtovg::CompositeOperation,
    renderables::{
        rect::InstanceBuilder as RectInstanceBuilder, Image, RadialGradient, Rect, Renderable,
    },
    size, size_pct,
    style::Styled,
    widgets::{Div, Svg},
    Color, Node, Pos, Scale, AABB,
};

pub struct Overlay {
    unlock_pressing_time: u128,
}

impl std::fmt::Debug for Overlay {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("Overlay")
            .field("unlock_pressing_time", &self.unlock_pressing_time)
            .finish()
    }
}

impl Overlay {
    pub fn new(unlock_pressing_time: u128) -> Self {
        Self {
            unlock_pressing_time: unlock_pressing_time,
        }
    }
}

impl Component for Overlay {
    fn props_hash(&self, hasher: &mut mctk_core::component::ComponentHasher) {
        self.unlock_pressing_time.hash(hasher);
    }

    fn render(&mut self, context: RenderContext) -> Option<Vec<Renderable>> {
        let width = context.aabb.width();
        //height of status bar 34.
        let height = context.aabb.height() + 34.;
        let AABB { pos, .. } = context.aabb;
        let mut rs = vec![];

        let background = RectInstanceBuilder::default()
            .pos(pos)
            .scale(Scale { width, height })
            .color(Color::rgba(0., 0., 0., 0.6))
            .build()
            .unwrap();

        let trasparent_scale = Scale::default() + self.unlock_pressing_time as f32 * 0.55;
        let transparent_pos = Pos::from([
            width / 2. - trasparent_scale.width / 2.,
            height / 2. - trasparent_scale.height / 2.,
        ]);
        let transparent_background = RectInstanceBuilder::default()
            .pos(transparent_pos)
            .scale(trasparent_scale)
            .color(Color::RED)
            .radius((40., 40., 40., 40.))
            .composite_operation(CompositeOperation::DestinationOut)
            .build()
            .unwrap();
        let image = Image::new(pos, Scale { width, height }, "background")
            .composite_operation(CompositeOperation::DestinationOver);

        let img_background = RectInstanceBuilder::default()
            .pos(pos)
            .scale(Scale { width, height })
            .color(Color::rgb(0., 0., 0.))
            .composite_operation(CompositeOperation::DestinationOver)
            .build()
            .unwrap();

        let radius = (
            (0. + self.unlock_pressing_time as f32 * 0.10)
                .min(height / 5.)
                .max(0.),
            (self.unlock_pressing_time as f32 * 0.30)
                .min(height)
                .max(0.),
        );
        let gradient = RadialGradient::new(
            Pos::from([width / 2., height / 2.]),
            radius,
            vec![
                (0.0, Color::rgba(5., 7., 10., 1.)),
                (0.5, Color::rgba(9., 12., 16., 0.2)),
                (1., Color::rgba(15., 17., 20., 0.)),
            ],
        );
        rs.push(Renderable::Rect(Rect::from_instance_data(background)));
        rs.push(Renderable::Rect(Rect::from_instance_data(
            transparent_background,
        )));
        rs.push(Renderable::RadialGradient(gradient));
        rs.push(Renderable::Image(image));
        // rs.push(Renderable::Rect(Rect::from_instance_data(img_background)));

        Some(rs)
    }

    // fn view(&self) -> Option<Node> {
    //     Some(node!(
    //         Div::new().bg(Color::rgba(0., 0., 0., 0.8)),
    //         lay![
    //             size: [480, 430],
    //             position_type: PositionType::Absolute,
    //             z_index_increment: 1000.0,
    //             axis_alignment: Center,
    //             cross_alignment: Center,
    //         ]
    //     ))
    // }
}
