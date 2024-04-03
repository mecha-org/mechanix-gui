use mctk_core::{
    component::{Component, Message},
    event::{Click, Event, MouseDown, MouseUp},
    lay, node, rect, size, size_pct,
    style::Styled,
    widgets::{Div, Svg},
    Color, Node,
};

pub struct UnlockButton {
    unlock_pressing_time: u128,
    on_press: Option<Box<dyn Fn() -> Message + Send + Sync>>,
    on_release: Option<Box<dyn Fn() -> Message + Send + Sync>>,
}

impl std::fmt::Debug for UnlockButton {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UnlockButton")
            .field("unlock_pressing_time", &self.unlock_pressing_time)
            .finish()
    }
}

impl UnlockButton {
    pub fn new(unlock_pressing_time: u128) -> Self {
        Self {
            unlock_pressing_time: unlock_pressing_time,
            on_press: None,
            on_release: None,
        }
    }

    pub fn on_press(mut self, on_press: Box<dyn Fn() -> Message + Send + Sync>) -> Self {
        self.on_press = Some(on_press);
        self
    }
    pub fn on_release(mut self, on_release: Box<dyn Fn() -> Message + Send + Sync>) -> Self {
        self.on_release = Some(on_release);
        self
    }
}

impl Component for UnlockButton {
    fn on_mouse_down(&mut self, event: &mut Event<MouseDown>) {
        if let Some(f) = &self.on_press {
            event.emit(f());
        }
    }

    fn on_mouse_up(&mut self, event: &mut Event<MouseUp>) {
        if let Some(f) = &self.on_release {
            event.emit(f());
        }
    }

    fn on_touch_down(&mut self, event: &mut Event<mctk_core::event::TouchDown>) {
        if let Some(f) = &self.on_press {
            event.emit(f());
        }
    }

    fn on_touch_up(&mut self, event: &mut Event<mctk_core::event::TouchUp>) {
        if let Some(f) = &self.on_release {
            event.emit(f());
        }
    }

    fn view(&self) -> Option<Node> {
        let unlock_pressing = self.unlock_pressing_time > 0;

        let unlock_icon = if unlock_pressing {
            "unlock_icon"
        } else {
            "lock_icon"
        };

        let background = if unlock_pressing {
            Color::WHITE
        } else {
            Color::BLACK
        };

        let border_color = if unlock_pressing {
            Color::rgb(0., 0., 0.)
        } else {
            Color::rgb(81., 81., 81.)
        };

        Some(
            node!(
                Div::new()
                    .bg(background)
                    .border(border_color, 2., (14., 14., 14., 14.)),
                lay![
                     size: [64, 64],
                     axis_alignment: Center,
                     cross_alignment: Center,
                ]
            )
            .push(node!(
                Svg::new(unlock_icon),
                lay![
                    size: [38, 38],
                ],
            )),
        )
    }
}
