// working switch
use gpui::{prelude::FluentBuilder, *};
use std::{rc::Rc, time::Duration};

use crate::prelude::{AMBER_600, DARK_NEUTRAL_100, DARK_NEUTRAL_900};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SwitchSize {
    XSmall,
    Small,
    Medium,
    Large,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    Left,
    Right,
}

impl Side {
    pub fn is_left(&self) -> bool {
        matches!(self, Side::Left)
    }
}

/// A Switch element that can be toggled on or off.
#[derive(IntoElement)]
pub struct Switch {
    id: ElementId,
    style: StyleRefinement,
    checked: bool,
    disabled: bool,
    label: Option<SharedString>,
    label_side: Side,
    on_click: Option<Rc<dyn Fn(&bool, &mut Window, &mut App)>>,
    size: SwitchSize,
    tooltip: Option<SharedString>,
}

impl Switch {
    /// Create a new Switch element.
    pub fn new(id: impl Into<ElementId>) -> Self {
        let id: ElementId = id.into();
        Self {
            id,
            style: StyleRefinement::default(),
            checked: false,
            disabled: false,
            label: None,
            on_click: None,
            label_side: Side::Right,
            size: SwitchSize::Medium,
            tooltip: None,
        }
    }

    /// Set the checked state of the switch.
    pub fn checked(mut self, checked: bool) -> Self {
        self.checked = checked;
        self
    }

    /// Set the label of the switch.
    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Add a click handler for the switch.
    pub fn on_click<F>(mut self, handler: F) -> Self
    where
        F: Fn(&bool, &mut Window, &mut App) + 'static,
    {
        self.on_click = Some(Rc::new(handler));
        self
    }

    /// Set tooltip for the switch.
    pub fn tooltip(mut self, tooltip: impl Into<SharedString>) -> Self {
        self.tooltip = Some(tooltip.into());
        self
    }

    /// Set the size of the switch.
    pub fn size(mut self, size: SwitchSize) -> Self {
        self.size = size;
        self
    }

    /// Set the disabled state of the switch.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Set the label side (left or right).
    pub fn label_side(mut self, side: Side) -> Self {
        self.label_side = side;
        self
    }
}

impl Styled for Switch {
    fn style(&mut self) -> &mut gpui::StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for Switch {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let checked = self.checked;
        let on_click = self.on_click.clone();
        let toggle_state = window.use_keyed_state(self.id.clone(), cx, |_, _| checked);

        // Color definitions
        let primary_color = gpui::rgb(AMBER_600);
        let bg_color = gpui::rgb(DARK_NEUTRAL_900);
        let thumb_color = gpui::rgb(DARK_NEUTRAL_900);
        let thumb_color_unchecked = gpui::rgb(0x323232);
        let transparent = gpui::rgba(0x00000000);

        let (bg, toggle_bg) = match checked {
            true => (primary_color, thumb_color),
            false => (bg_color, thumb_color_unchecked),
        };

        let (bg, toggle_bg) = if self.disabled {
            (if checked { bg } else { bg }, toggle_bg)
        } else {
            (bg, toggle_bg)
        };

        let (bg_width, bg_height) = match self.size {
            SwitchSize::XSmall | SwitchSize::Small => (px(28.), px(16.)),
            _ => (px(33.75), px(20.25)),
        };
        let bar_width = match self.size {
            SwitchSize::XSmall | SwitchSize::Small => px(12.),
            _ => px(11.25),
        }; // for thumb

        let inset = px(2.);
        let radius = bg_height; // Full rounded

        div().child(
            div()
                .id(self.id.clone())
                .flex()
                .flex_row()
                .gap(px(8.))
                .items_start()
                .when(self.label_side.is_left(), |this| {
                    this.flex_row().flex_row_reverse()
                })
                .child(
                    // Switch Bar
                    div()
                        .id(self.id.clone())
                        .w(bg_width)
                        .h(bg_height)
                        .rounded(radius)
                        .flex()
                        .items_center()
                        .border(px(inset.into()))
                        .border_color(if checked {
                            transparent
                        } else {
                            rgb(DARK_NEUTRAL_100)
                        })
                        .bg(bg)
                        .child(
                            // Switch Toggle
                            div()
                                .rounded(radius)
                                .bg(toggle_bg)
                                .shadow_md()
                                .size(bar_width)
                                .border(if checked { px(0.) } else { px(2.) })
                                .border_color(if checked {
                                    transparent
                                } else {
                                    rgb(DARK_NEUTRAL_100)
                                })
                                .map(|this| {
                                    let prev_checked = toggle_state.read(cx);
                                    if !self.disabled && *prev_checked != checked {
                                        let duration = Duration::from_secs_f64(0.15);
                                        cx.spawn({
                                            let toggle_state = toggle_state.clone();
                                            async move |cx| {
                                                cx.background_executor().timer(duration).await;
                                                _ = toggle_state
                                                    .update(cx, |this, _| *this = checked);
                                            }
                                        })
                                        .detach();

                                        this.with_animation(
                                            ElementId::NamedInteger("move".into(), checked as u64),
                                            Animation::new(duration),
                                            move |this, delta| {
                                                let max_x = bg_width - bar_width - inset * 2.;
                                                let x = if checked {
                                                    max_x * delta
                                                } else {
                                                    max_x - max_x * delta
                                                };
                                                this.left(x)
                                            },
                                        )
                                        .into_any_element()
                                    } else {
                                        let max_x = bg_width - bar_width - inset * 2.;
                                        let x = if checked { max_x } else { px(2.) };
                                        this.left(x).into_any_element()
                                    }
                                }),
                        ),
                )
                .when_some(self.label, |this, label| {
                    this.child(div().line_height(bg_height).child(label).map(
                        |this| match self.size {
                            SwitchSize::XSmall | SwitchSize::Small => this.text_sm(),
                            _ => this.text_base(),
                        },
                    ))
                })
                .when_some(
                    on_click
                        .as_ref()
                        .map(|c| c.clone())
                        .filter(|_| !self.disabled),
                    |this, on_click| {
                        let toggle_state = toggle_state.clone();
                        this.on_mouse_down(gpui::MouseButton::Left, move |_, window, cx| {
                            cx.stop_propagation();
                            _ = toggle_state.update(cx, |this, _| *this = checked);
                            on_click(&!checked, window, cx);
                        })
                    },
                ),
        )
    }
}
