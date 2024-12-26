use crate::{components::*, radio_node, screens::battery::battery_model::BatteryModel};

#[derive(Debug)]
pub struct PerformanceMode {}
impl Component for PerformanceMode {
    fn view(&self) -> Option<Node> {
        let mut base: Node = node!(
            widgets::Div::new(),
            lay![
                size_pct: [100],
                direction: Direction::Column,
            ]
        );

        let mut main_node = node!(
            widgets::Div::new(),
            lay![
                size_pct: [100, 90],
                cross_alignment: layout::Alignment::Stretch,
                direction: layout::Direction::Column,
                margin: [10., 0., 0., 0.],
            ]
        );

        let modes: Vec<String> = BatteryModel::get().available_modes.get().clone();
        let current_mode = BatteryModel::get().cureent_mode.get().clone();

        let mut available_modes_txt = vec![];

        for (i, mode) in modes.into_iter().enumerate() {
            available_modes_txt.push((txt!(mode.clone()), txt!(mode.clone())));
        }

        let sub_header = node!(
            Div::new(),
            lay![
                margin: [0., 8., 0., 8.]
            ]
        )
        .push(sub_header_node("Performance Mode"));

        main_node = main_node.push(sub_header);
        main_node = main_node.push(radio_node!(
            available_modes_txt,
            txt!(current_mode),
            Box::new(|x| msg!(BatteryModel::set_mode(&x)))
        ));
        main_node = main_node.push(node!(
            Div::new().bg(Color::TRANSPARENT),
            lay![size_pct: [100, 8]]
        ));
        main_node = main_node.push(
            node!(
                Div::new().bg(Color::TRANSPARENT),
                lay![
                    size_pct: [100, 60],
                    direction: Direction::Column,
                    cross_alignment: Alignment::Stretch,
                    axis_alignment: Alignment::Start,
                ]
            )
            .push(
                node!(
                    Div::new(),
                    lay![
                        size_pct: [100, 15],
                        direction: Direction::Row,
                        axis_alignment: Alignment::Start,
                    ]
                )
                .push(
                    node!(
                        Div::new().bg(Color::TRANSPARENT),
                        lay![
                            size_pct: [5, 100],
                            axis_alignment: Alignment::Start,
                        ]
                    )
                    .push(get_text_node("**", Color::RED)),
                )
                .push(
                    node!(
                        Div::new(),
                        lay![
                            size_pct: [95, 100],
                            axis_alignment: Alignment::Start,
                        ]
                    )
                    .push(get_text_node(
                        "Higher performance will use battery faster and ",
                        Color::rgb(197.0, 197.0, 197.0),
                    )),
                ),
            )
            .push(
                node!(
                    Div::new(),
                    lay![
                        size_pct: [100, 15],
                        direction: Direction::Row,
                        axis_alignment: Alignment::Start,
                    ]
                )
                .push(
                    node!(
                        Div::new(),
                        lay![
                            size_pct: [100, 100],
                            axis_alignment: Alignment::Start,
                        ]
                    )
                    .push(get_text_node(
                        "increase the temperature of the device significantly.",
                        Color::rgb(197.0, 197.0, 197.0),
                    )),
                ),
            )
            .push(
                node!(
                    Div::new(),
                    lay![
                        size_pct: [100, 20],
                        direction: Direction::Row,
                        axis_alignment: Alignment::Start,
                    ]
                )
                .push(
                    node!(
                        Div::new(),
                        lay![
                            size_pct: [100, 100],
                            axis_alignment: Alignment::Start,
                        ]
                    )
                    .push(get_text_node(
                        "Check ambient temperature before proceeding.",
                        Color::rgb(197.0, 197.0, 197.0),
                    )),
                ),
            ),
        );
        base = base.push(main_node);
        Some(base)
    }
}

pub fn get_text_node(text: &str, color: Color) -> Node {
    let text_node = node!(
        widgets::Text::new(txt!(text))
            .style("color", color)
            .style("size", 16.0)
            .style("line_height", 22.)
            .style("font", "Space Grotesk")
            .style("font_weight", FontWeight::Medium),
        lay![
        margin: [5.0, 0.0, 5.0, 0.0]
        ]
    );
    text_node
}
