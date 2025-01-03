use crate::{components::*, main, radio_node, screens::battery::battery_model::BatteryModel};

#[derive(Debug)]
pub struct PerformanceMode {}
impl Component for PerformanceMode {
    fn view(&self) -> Option<Node> {
        let mut base: Node = node!(
            widgets::Div::new(),
            lay![
                size_pct: [100],
                direction: Direction::Column,
                padding: [5.0, 0.0, 5.0, 0.0],
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

        let mut scrollable_node = node!(
            Scrollable::new(size!(440, 280)),
            lay![
                size: [440, 280],
            ]
        )
        .push(node!(
            Div::new(),
            lay![
                size: [440, Auto],
                direction: Direction::Column,
                cross_alignment: Alignment::Stretch,
            ]
        ));

        let sub_header = node!(
            Div::new(),
            lay![
                margin: [0., 10., 0., 8.]
            ]
        )
        .push(sub_header_node("Performance Mode"));

        let modes: Vec<String> = BatteryModel::get().available_modes.get().clone();
        let current_mode = BatteryModel::get().cureent_mode.get().clone();

        let mut available_modes_txt = vec![];

        for (i, mode) in modes.into_iter().enumerate() {
            available_modes_txt.push((txt!(mode.clone()), txt!(mode.clone())));
        }

        scrollable_node = scrollable_node.push(radio_node!(
            available_modes_txt,
            txt!(current_mode),
            Box::new(|x| msg!(BatteryModel::set_mode(&x)))
        ));
        scrollable_node = scrollable_node.push(node!(
            Div::new().bg(Color::TRANSPARENT),
            lay![size_pct: [100, 8]]
        ));
        scrollable_node = scrollable_node.push(
            node!(
                Div::new().bg(Color::TRANSPARENT),
                lay![
                    size_pct: [100, Auto],
                    direction: Direction::Column,
                    cross_alignment: Alignment::Stretch,
                    axis_alignment: Alignment::Start,
                    margin: [0., 8., 0., 8.]
                ]
            )
            // .push(
            //     node!(
            //         Div::new(),
            //         lay![
            //             size_pct: [100, 20],
            //             direction: Direction::Row,
            //             axis_alignment: Alignment::Start,
            //         ]
            //     )
            //     .push(
            //         node!(
            //             Div::new().bg(Color::TRANSPARENT),
            //             lay![
            //                 size_pct: [5, 100],
            //                 axis_alignment: Alignment::Start,
            //             ]
            //         )
            //         .push(get_text_node("**", Color::RED)),
            //     )
            //     .push(
            //         node!(
            //             Div::new(),
            //             lay![
            //                 size_pct: [95, 100],
            //                 axis_alignment: Alignment::Start,
            //             ]
            //         )
            //         .push(get_text_node(
            //             "Higher performance will use battery",
            //             Color::rgb(197.0, 197.0, 197.0),
            //         )),
            //     ),
            // )
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
                        "Higher performance will use battery",
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
                        "faster & increase the temperature of ",
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
                        "the device significantly. Check ambient",
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
                        "temperature before proceeding.",
                        Color::rgb(197.0, 197.0, 197.0),
                    )),
                ),
            ),
        );

        main_node = main_node.push(sub_header);
        main_node = main_node.push(scrollable_node);
        base = base.push(main_node);
        Some(base)
    }
}

pub fn get_text_node(text: &str, color: Color) -> Node {
    let text_node = node!(
        widgets::Text::new(txt!(text))
            .style("color", color)
            .style("font", "Inter")
            .with_class("text-xl leading-6 font-normal"),
        lay![
        margin: [5.0, 0.0, 5.0, 0.0]
        ]
    );
    text_node
}
