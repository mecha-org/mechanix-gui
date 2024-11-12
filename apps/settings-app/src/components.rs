use crate::{
    gui::{Message, Routes},
    shared::h_divider::HDivider,
};
pub use mctk_core::component::*;
pub use mctk_core::layout::*;
pub use mctk_core::style::*;
pub use mctk_core::widgets::*;
pub use mctk_core::*;

pub fn footer_node() -> Node {
    let mut footer_div = node!(
        Div::new()
            .style("background_color", Color::BLACK)
            .bg(Color::BLACK),
        lay![
            size_pct: [110, 20],
            direction: Direction::Column,
            cross_alignment: Alignment::Stretch,
            axis_alignment: Alignment::End,
            position_type: Absolute,
            z_index: 10,
            position: [Auto, 0.0, 0.0, 0.0],
        ]
    );
    let footer_row: Node = node!(
        Div::new(),
        lay![
            size_pct: [100],
            direction: Direction::Row,
            axis_alignment: Alignment::Stretch,
        ]
    )
    .push(
        node!(
            Div::new(),
            lay![
                size_pct: [50],
                axis_alignment: Alignment::Start,
                cross_alignment: Alignment::Center,
            ],
        )
        .push(
            node!(
                Div::new(),
                lay![
                    padding: [10., 15., 0., 0.]
                ]
            )
            .push(node!(
                IconButton::new("right_arrow_icon")
                    .on_click(Box::new(|| msg!(Message::ChangeRoute {
                        route: Routes::SettingsList
                    })))
                    .icon_type(IconType::Png)
                    .style(
                        "size",
                        Size {
                            width: Dimension::Px(52.0),
                            height: Dimension::Px(52.0),
                        }
                    )
                    .style("background_color", Color::TRANSPARENT)
                    .style("border_color", Color::TRANSPARENT)
                    .style("active_color", Color::rgba(85., 85., 85., 0.50))
                    .style("padding", 8.)
                    .style("radius", 12.),
                lay![
                    size: [30, 30],
                ]
            )),
        ),
    );

    footer_div = footer_div
        .push(node!(HDivider { size: 1. }))
        .push(footer_row);
    footer_div
}

pub fn header_node(text: &str) -> Node {
    let mut header = node!(
        widgets::Div::new(),
        // Div::new().bg(Color::MID_GREY),
        lay![
            size_pct: [100, 25],
            axis_alignment: layout::Alignment::Stretch,
            // cross_alignment: Alignment::Center,
            cross_alignment: layout::Alignment::Stretch,
            direction: layout::Direction::Column,
            padding: [5.0, 0.0, 0.0, 0.0],
            margin: [25., 0., 0., 10.]
        ]
    );
    let header_text = node!(
        widgets::Text::new(txt!(text))
            .style("font", "Space Grotesk")
            .style("size", 28.)
            .style("color", Color::rgb(197.0, 197.0, 197.0))
            .style("font_weight", style::FontWeight::Normal),
        lay![
            margin:[2.0, 5.0, 2.0, 5.0],
            size: size!(20.0, 50.0),
            axis_alignment: layout::Alignment::Stretch,
        ]
    );
    header = header.push(header_text);
    header
}

pub fn text_bold_node(text: &str) -> Node {
    let text_node = node!(
        widgets::Text::new(txt!(text))
            .style("color", Color::WHITE)
            .style("size", 20.0)
            .style("line_height", 20.)
            .style("font", "Space Grotesk")
            .style("font_weight", FontWeight::Bold),
        lay![margin: [0.0, 10.0, 0.0, 10.0]]
    );
    text_node
}

pub fn text_node(text: &str) -> Node {
    let text_node = node!(
        widgets::Text::new(txt!(text))
            .style("color", Color::rgb(197.0, 197.0, 197.0))
            .style("size", 20.0)
            .style("line_height", 20.)
            .style("font", "Space Grotesk")
            .style("font_weight", FontWeight::Medium),
        lay![margin: [0.0, 10.0, 0.0, 10.0]]
    );
    text_node
}

pub fn icon_node(name: &str) -> Node {
    let icon_node = node!(
        widgets::Image::new(name.to_string()),
        lay![
            size: [20, 20],
            margin: [0, 5., 0, 5.],
            padding: [0., 0., 0., 0.]
        ]
    );
    icon_node
}

#[macro_export]
macro_rules! tab_item_node {
    ([$($left_nodes:expr),* $(,)?], [$($right_nodes:expr),* $(,)?], on_click = $on_click:expr) => {{
        todo!("ON CLICK");
        let left_nodes = vec![$($left_nodes),*];
        let right_nodes = vec![$($right_nodes),*];
        let mut base = node!(
            Div::new(),
            lay![
                padding: [15, 0, 15, 0],
                size_pct: [100],
                direction: Direction::Row,
                axis_alignment: Alignment::Stretch,
            ]
        );

        let mut left = node!(
            Div::new(),
            lay![
                size_pct: [50],
                axis_alignment: Alignment::Start
            ],
        );
        for node in left_nodes {
            left = left.push(node);
        }

        let mut right = node!(
            Div::new(),
            lay![
                size_pct: [50],
                axis_alignment: Alignment::End
            ],
        );
        for node in right_nodes {
            right = right.push(node);
        }
        base = base.push(left);
        base = base.push(right);
        base
    }};
    ([$($left_nodes:expr),* $(,)?], [$($right_nodes:expr),* $(,)?]) => {{
        let left_nodes = vec![$($left_nodes),*];
        let right_nodes = vec![$($right_nodes),*];
        let mut base = node!(
            Div::new(),
            lay![
                padding: [20, 0, 20, 0],
                size_pct: [100, Auto],
                direction: Direction::Row,
                axis_alignment: Alignment::Stretch,
            ]
        );

        let mut left = node!(
            Div::new(),
            lay![
                size_pct: [50],
                axis_alignment: Alignment::Start
            ],
        );
        for node in left_nodes {
            left = left.push(node);
        }

        let mut right = node!(
            Div::new(),
            lay![
                size_pct: [50],
                axis_alignment: Alignment::End
            ],
        );
        for node in right_nodes {
            right = right.push(node);
        }
        base = base.push(left);
        base = base.push(right);
        base
    }};
}

// #[derive(Default)]
// pub struct TabItemState {
//     left: Option<Node>,
//     right: Option<Node>,
//     on_click: Option<Box<dyn Fn() -> Message + Send + Sync>>,
// }
//
// impl std::fmt::Debug for TabItemState {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         f.debug_struct("TabItemState").finish()
//     }
// }
//
// #[component(State = "TabItemState")]
// pub struct TabItemComponent {}
//
// impl std::fmt::Debug for TabItemComponent {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         f.debug_struct("TabItemComponent").finish()
//     }
// }
//
// #[state_component_impl(TabItemState)]
// impl Component for TabItemComponent {
//     fn on_click(&mut self, event: &mut event::Event<event::Click>) {
//         let on_click = &self.state_ref().on_click;
//         if let Some(f) = on_click {
//             event.emit(f());
//         }
//     }
//
//     fn view(&self) -> Option<Node> {
//         let mut base = node!(
//             Div::new(),
//             lay![
//                 padding: [15, 5, 15, 5],
//                 size_pct: [100],
//                 direction: Direction::Row,
//                 axis_alignment: Alignment::Stretch,
//             ]
//         );
//         // base = base.push(self.state_ref().left?);
//         // base = base.push(self.state_ref().right?);
//         Some(base)
//     }
// }
//
// impl TabItemComponent {
//     pub fn new(
//         left_nodes: Vec<Node>,
//         right_nodes: Vec<Node>,
//         on_click: Option<Box<dyn Fn() -> Message + Send + Sync>>,
//     ) -> Self {
//         let mut left = node!(
//             Div::new(),
//             lay![
//                 size_pct: [50],
//                 axis_alignment: Alignment::Start
//             ],
//         );
//         for node in left_nodes {
//             left = left.push(node);
//         }
//
//         let mut right = node!(
//             Div::new(),
//             lay![
//                 size_pct: [50],
//                 axis_alignment: Alignment::End
//             ],
//         );
//         for node in right_nodes {
//             right = right.push(node);
//         }
//
//         Self {
//             dirty: false,
//             state: Some(TabItemState {
//                 left: Some(left),
//                 right: Some(right),
//                 on_click,
//             }),
//         }
//     }
// }
