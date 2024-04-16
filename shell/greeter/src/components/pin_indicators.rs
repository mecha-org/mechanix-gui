use mctk_core::{
    component::Component, lay, node, rect, size, style::Styled, widgets::Div, Color, Node,
};

pub const MAX_PIN_LENGTH: usize = 4;

#[derive(Debug)]
pub struct PinIndicators {
    pub pin_length: usize,
}

impl Component for PinIndicators {
    fn view(&self) -> Option<Node> {
        let mut indicators = node!(Div::new(), lay![],);

        for i in 1..MAX_PIN_LENGTH + 1 {
            let is_filled = i <= self.pin_length;

            let indicator = node!(
                Div::new()
                    .bg(if is_filled {
                        Color::WHITE
                    } else {
                        Color::TRANSPARENT
                    })
                    .border(Color::WHITE, 2., (5., 5., 5., 5.)),
                lay![
                    size: [10, 10],
                    margin: [0., 8.]
                ],
            )
            .key(i as u64);
            indicators = indicators.push(indicator);
        }

        Some(indicators)
    }
}
