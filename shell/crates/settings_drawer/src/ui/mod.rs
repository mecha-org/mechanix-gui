mod icon;
mod widgets;
use gpui::*;
use icon::IconName;
use widgets::IconButton;

pub struct SettingsDrawer {
    pub rotation_on: bool,
}

impl SettingsDrawer {
    pub fn new() -> Self {
        Self { rotation_on: true }
    }
}

impl Render for SettingsDrawer {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .bg(rgb(0x101010))
            .pt_12()
            .pl_8()
            .pr_8()
            .w_full()
            .h_full()
            .content_stretch()
            .gap_4()
            .child(div().flex().h(px(32.82)).bg(rgb(0x181818)).rounded(px(12.)))
            .child(
                div()
                    .grid()
                    .grid_rows(2)
                    .grid_cols(4)
                    .h(px(236.))
                    .bg(rgb(0x181818))
                    .p(px(20.))
                    .rounded(px(12.))
                    .child(
                        IconButton::new("rotation")
                            .icon(if self.rotation_on {
                                IconName::RotationOn
                            } else {
                                IconName::RotationOff
                            })
                            .on_click(cx.listener(
                                |this: &mut SettingsDrawer,
                                 _event: &ClickEvent,
                                 _window: &mut Window,
                                 cx: &mut Context<Self>| {
                                    this.rotation_on = !this.rotation_on;
                                    cx.notify();
                                },
                            )),
                    )
                    .child(
                        IconButton::new("airplane")
                            .icon(IconName::Airplane)
                            .icon_color(rgb(0xF4F4F4))
                            .on_click(cx.listener(|_, _, _, _| {
                                println!("Airplane button clicked");
                            })),
                    )
                    .child(
                        IconButton::new("3")
                            .icon(IconName::ScreenMirroringOff)
                            .icon_color(rgb(0x4D4D4D))
                            .on_click(cx.listener(|_, _, _, _| {
                                println!("Button 3 clicked");
                            })),
                    )
                    .child(
                        IconButton::new("4")
                            .icon(IconName::Battery40)
                            .icon_color(rgb(0x4D4D4D))
                            .on_click(cx.listener(|_, _, _, _| {
                                println!("Button 4 clicked");
                            })),
                    )
                    .child(
                        IconButton::new("5")
                            .icon(IconName::MicroPhoneOff)
                            .icon_color(rgb(0x4D4D4D))
                            .on_click(cx.listener(|_, _, _, _| {
                                println!("Button 5 clicked");
                            })),
                    )
                    .child(
                        IconButton::new("6")
                            .icon(IconName::MicroPhoneOff)
                            .icon_color(rgb(0x4D4D4D))
                            .on_click(cx.listener(|_, _, _, _| {
                                println!("Button 6 clicked");
                            })),
                    )
                    .child(
                        IconButton::new("7")
                            .icon(IconName::Calculator)
                            .icon_color(rgb(0xF4F4F4))
                            .on_click(cx.listener(|_, _, _, _| {
                                println!("Button 7 clicked");
                            })),
                    )
                    .child(
                        IconButton::new("8")
                            .icon(IconName::Camera)
                            .icon_color(rgb(0xF4F4F4))
                            .on_click(cx.listener(|_, _, _, _| {
                                println!("Button 8 clicked");
                            })),
                    ),
            )
            .child(div().flex().h(px(104.)).bg(rgb(0x181818)).rounded(px(12.)))
            .child(div().flex().h(px(104.)).bg(rgb(0x181818)).rounded(px(12.)))
            .child(
                div()
                    .flex()
                    .absolute()
                    .bottom(px(19.))
                    .left(px(210.))
                    .w(px(120.))
                    .h(px(4.))
                    .rounded(px(4.))
                    .bg(rgb(0x797979)),
            )
    }
}
