use crate::{
    gui::{Message, Routes},
    shared::h_divider::HDivider,
};
pub use mctk_core::component::*;
pub use mctk_core::layout::*;
pub use mctk_core::style::*;
pub use mctk_core::widgets::*;
pub use mctk_core::*;

#[derive(Default, Debug, Clone)]
pub struct ScreenRoute {
    pub(crate) route: Routes,
}

#[macro_export]
macro_rules! header_node {
    ($title:expr, $back_on_click:expr) => {{
        let text_node = node!(
            Text::new(txt!($title))
                .style("color", Color::WHITE)
                .style("size", 28.0)
                .style("line_height", 20.)
                .style("font", "Space Grotesk")
                .style("font_weight", FontWeight::Medium),
            lay![
                size_pct: [100, Auto],
            ]
        );
        let header_node = node!(
            Div::new(),
            lay![
                size_pct: [100, 15],
                direction: Direction::Row,
                axis_alignment: Alignment::Stretch,
                cross_alignment: Alignment::Center,
                margin: [0., 0., 5., 0.],
                // position_type: Absolute,
                position: [0., 0., Auto, 0.],
            ]
        )
        .push(
            node!(
                Div::new(),
                lay![
                    size_pct: [80, Auto],
                    axis_alignment: Alignment::Start,
                    cross_alignment: Alignment::Center,
                ],
            )
            .push(node!(
                IconButton::new("back_icon")
                    .on_click($back_on_click)
                    .icon_type(IconType::Png)
                    .style(
                        "size",
                        Size {
                            width: Dimension::Px(34.0),
                            height: Dimension::Px(34.0),
                        }
                    )
                    .style("background_color", Color::TRANSPARENT)
                    .style("border_color", Color::TRANSPARENT)
                    .style("active_color", Color::rgba(85., 85., 85., 0.50))
                    .style("radius", 10.),
                lay![
                    size: [52, 52],
                    padding: [0, 0, 0, 20.],
                    axis_alignment: Alignment::Start,
                    cross_alignment: Alignment::Center,
                ]
            ))
            .push(text_node),
        )
        .push(
            node!(
                Div::new(),
                lay![
                    size_pct: [20, Auto],
                    axis_alignment: Alignment::End
                ]
            )
            // .push(node!(
            //     widgets::Image::new("network_settings_icon"),
            //     lay![
            //         size: [24, 24],
            //     ]
            // )),
        );
        header_node
    }};
    ($title:expr, $back_on_click:expr, $right_icon:expr) => {{
        let text_node = node!(
            Text::new(txt!($title))
                .style("color", Color::WHITE)
                .style("size", 28.0)
                .style("line_height", 20.)
                .style("font", "Space Grotesk")
                .style("font_weight", FontWeight::Medium),
            lay![
                size_pct: [100, Auto],
            ]
        );
        let header_node = node!(
            Div::new(),
            lay![
                size_pct: [100, 10],
                direction: Direction::Row,
                axis_alignment: Alignment::Stretch,
                cross_alignment: Alignment::Center,
                margin: [0., 0., 5., 0.],
                // position_type: Absolute,
                position: [0., 0., Auto, 0.],
            ]
        )
        .push(
            node!(
                Div::new(),
                lay![
                    size_pct: [80, Auto],
                    axis_alignment: Alignment::Start,
                    cross_alignment: Alignment::Center,
                ],
            )
            .push(node!(
                IconButton::new("back_icon")
                    .on_click($back_on_click)
                    .icon_type(IconType::Png)
                    .style(
                        "size",
                        Size {
                            width: Dimension::Px(34.0),
                            height: Dimension::Px(34.0),
                        }
                    )
                    .style("background_color", Color::TRANSPARENT)
                    .style("border_color", Color::TRANSPARENT)
                    .style("active_color", Color::rgba(85., 85., 85., 0.50))
                    .style("radius", 10.),
                lay![
                    size: [52, 52],
                    padding: [0, 0, 0, 20.],
                    axis_alignment: Alignment::Start,
                    cross_alignment: Alignment::Center,
                ]
            ))
            .push(text_node),
        )
        .push(
            node!(
                Div::new(),
                lay![
                    size_pct: [20, Auto],
                    axis_alignment: Alignment::End
                ]
            )
            .push(node!(
                widgets::Image::new($right_icon),
                lay![
                    size: [24, 24],
                ]
            )),
        );
        header_node
    }};
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
            margin: [0., 0., 0., 10.]
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
    (
        [$($left_nodes:expr),* $(,)?],
        [$($right_nodes:expr),* $(,)?],
        on_click: $on_click:expr
        $(,)?
    ) => {{
        let left_nodes = vec![$($left_nodes),*];
        let right_nodes = vec![$($right_nodes),*];
        let mut base = node!(
            TabItemComponent {
                on_click: $on_click
            },
        );
        let mut left = node!(
            Div::new(),
            lay![
                size_pct: [50],
                cross_alignment: Alignment::Center,
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
                cross_alignment: Alignment::Center,
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
    (
        [$($left_nodes:expr),* $(,)?],
        [$($right_nodes:expr),* $(,)?],
        route: $route:expr
        $(,)?
    ) => {{
        let left_nodes = vec![$($left_nodes),*];
        let right_nodes = vec![$($right_nodes),*];
        let mut base = node!(
            TabItemComponent {
                on_click: Some(Box::new(move || msg!(Message::ChangeRoute {
                    route: $route
                }))),
            },
        );
        let mut left = node!(
            Div::new(),
            lay![
                size_pct: [50],
                cross_alignment: Alignment::Center,
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
                cross_alignment: Alignment::Center,
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
    (
        [$($left_nodes:expr),* $(,)?],
        [$($right_nodes:expr),* $(,)?]
        $(,)?
    ) => {{
        let left_nodes = vec![$($left_nodes),*];
        let right_nodes = vec![$($right_nodes),*];
        let mut base = node!(
            TabItemComponent {
                on_click: None
            },
        );
        let mut left = node!(
            Div::new(),
            lay![
                size_pct: [50],
                cross_alignment: Alignment::Center,
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
                cross_alignment: Alignment::Center,
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

pub fn radio_node(options: Vec<&str>) -> Node {
    let options = options.into_iter().map(|x| txt!(x.to_string())).collect();
    let radio = node!(
        RadioButtons::new(options, 0,)
            .direction(mctk_core::layout::Direction::Column)
            .style("font_size", 18.0)
            .style("padding", 0.)
            //.multi_select(true)
            .max_columns(1),
        // .on_change(Box::new(|s| msg!(HelloEvent::RadioSelect { selection: s }))),
        lay![margin: [0], size: [450, Auto]]
    );
    radio
}

// #[derive(Default)]
// pub struct TabItemState {
//     left: Option<Node>,
//     right: Option<Node>,
//     on_click: Option<Box<dyn Fn() -> Message + Send + Sync>>,
// }

// impl std::fmt::Debug for TabItemState {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         f.debug_struct("TabItemState").finish()
//     }
// }

// #[component(State = "TabItemState")]
pub struct TabItemComponent {
    pub on_click: Option<Box<dyn Fn() -> Box<Message> + Send + Sync>>,
}

impl std::fmt::Debug for TabItemComponent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TabItemComponent").finish()
    }
}

// #[state_component_impl(TabItemState)]
impl Component for TabItemComponent {
    fn on_click(&mut self, event: &mut event::Event<event::Click>) {
        if let Some(f) = &self.on_click {
            event.emit(f());
        }
    }

    fn container(&self) -> Option<Vec<usize>> {
        Some(vec![0])
    }

    fn view(&self) -> Option<Node> {
        let base = node!(
            Div::new(),
            lay![
                padding: [20, 0, 20, 0],
                size_pct: [100, Auto],
                direction: Direction::Row,
                cross_alignment: Alignment::Center,
                axis_alignment: Alignment::Stretch,
            ]
        );
        // base = base.push(self.state_ref().left?);
        // base = base.push(self.state_ref().right?);
        Some(base)
    }
}

// -----------------

pub struct ClicableIconComponent {
    pub on_click: Option<Box<dyn Fn() -> Box<Message> + Send + Sync>>,
}

impl std::fmt::Debug for ClicableIconComponent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ClicableIconComponent").finish()
    }
}

impl Component for ClicableIconComponent {
    fn container(&self) -> Option<Vec<usize>> {
        Some(vec![0])
    }

    fn view(&self) -> Option<Node> {
        let base = node!(
            Div::new(),
            lay![
                size_pct: [50],
                cross_alignment: Alignment::Center,
                axis_alignment: Alignment::End
            ],
        );
        Some(base)
    }
}

// -----------------

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