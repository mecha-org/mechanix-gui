use crate::{widgets::HomescreenWidget, SystemUsageState};
use gpui::*;

pub struct SystemUsage {
    bounds: Bounds<Pixels>,
    color: Hsla,
    border_color: Hsla,
    has_border: bool,
}

impl SystemUsage {
    pub fn new(color: impl Into<Hsla>, border_color: impl Into<Hsla>, has_border: bool) -> Self {
        Self {
            bounds: Bounds::default(),
            color: color.into(),
            border_color: border_color.into(),
            has_border,
        }
    }
}

impl HomescreenWidget for SystemUsage {
    fn render(&self, cx: &mut gpui::App) -> gpui::AnyElement {
        let state = cx.global::<SystemUsageState>();
        let cpu_usage = state.cpu_usage.clone();
        let memory_usage = state.memory_usage.clone();
        let uptime = state.uptime.clone();

        div()
            .size_full()
            .flex()
            .flex_col()
            .justify_center()
            .items_center()
            .p(px(12.))
            .gap(px(16.))
            .child(div().child("System"))
            .child(
                div()
                    .flex()
                    .justify_between()
                    .child(
                        div()
                            .w_full()
                            .flex()
                            .flex_col()
                            .child(cpu_usage)
                            .child("CPU"),
                    )
                    .child(div().flex().flex_col().child(memory_usage).child("Memory")),
            )
            .child(div().flex().flex_col().child(uptime).child("Uptime"))
            .into_any_element()
    }

    fn set_bounds(&mut self, bounds: Bounds<Pixels>) {
        self.bounds = bounds;
    }

    fn get_bounds(&self) -> Bounds<Pixels> {
        self.bounds
    }

    fn background_color(&self) -> Hsla {
        self.color
    }

    fn has_border(&self) -> bool {
        self.has_border
    }

    fn border_color(&self) -> Hsla {
        self.border_color
    }
}
