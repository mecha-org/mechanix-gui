use gpui::{prelude::FluentBuilder, *};
use theme::ActiveTheme;

use crate::{
    layout::{KeyButton, ParsedLayout},
    trie::trie::Trie,
    types::{ActionParsed, Label},
};
pub struct OnScreenKeyboard {
    pub current_view: String,
    pub current_layout: ParsedLayout,
    pub key_pressed: Option<KeyButton>,
    pub suggestions: Vec<String>,
    pub suggested_for: String,
    pub trie: Trie,
}

impl OnScreenKeyboard {
    fn handle_key_press(
        &mut self,
        key: Option<KeyButton>,
        window: &mut Window,
        _cx: &mut Context<Self>,
    ) {
        if let Some(im) = window.get_input_method() {
            if let Some(key) = key {
                match key.action {
                    ActionParsed::Submit { text, keysym } => {
                        println!("committing {:?}", text);
                        im.commit_string(&text.unwrap());
                        im.commit();
                    }
                    _ => (),
                }
            }
        }
    }

    fn handle_suggestion_press(
        &mut self,
        suggestion: &str,
        window: &mut Window,
        _cx: &mut Context<Self>,
    ) {
        println!("suggestion: {}", suggestion);
        if let Some(im) = window.get_input_method() {
            im.delete_surrounding_text(self.suggested_for.len() as u32, 0);
            im.commit_string(suggestion);
            im.commit();
        }
    }
}

impl Render for OnScreenKeyboard {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let is_active = window.get_input_method().map_or(false, |im| im.is_active());
        println!("Keyboard is active: {}", is_active);

        let view = self
            .current_layout
            .get_view(self.current_view.as_str())
            .unwrap();
        let rows = view.rows.clone();
        let colors = cx.theme().colors.clone();
        let suggestions = self.suggestions.clone();

        div()
            .size_full()
            .flex()
            .flex_col()
            .child(
                div()
                    .w_full()
                    .h(px(36.))
                    .pl(px(12.))
                    .pr(px(12.))
                    .bg(colors.background_600)
                    .flex()
                    .flex_row()
                    .items_center()
                    .justify_between()
                    .text_color(colors.foreground_100)
                    .text_size(px(18.))
                    .font_weight(FontWeight(400.))
                    .children(suggestions.into_iter().map(|s| s)),
            )
            .child(
                div()
                    .id("click-area")
                    .w_full()
                    .h(px(226.))
                    .pt(px(1.))
                    .bg(colors.background_600)
                    .relative()
                    .flex()
                    .flex_col()
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(|this, event: &MouseDownEvent, window, cx| {
                            cx.stop_propagation();

                            let button = this.current_layout.find_button_at_position(
                                &this.current_view,
                                event.position.x.to_f64(),
                                event.position.y.to_f64() - 36.,
                            );
                            println!("Clicked button: {:#?}", button);
                            if let Some(button) = button {
                                match &button.action {
                                    ActionParsed::SetView(view) => this.current_view = view.clone(),
                                    ActionParsed::LockView {
                                        lock,
                                        unlock,
                                        latches,
                                        looks_locked_from,
                                    } => {
                                        if this.current_view == lock.clone() {
                                            this.current_view = unlock.clone();
                                        } else if this.current_view == unlock.clone() {
                                            this.current_view = lock.clone();
                                        }
                                    }
                                    ActionParsed::ApplyModifier(modifier_parsed) => todo!(),
                                    ActionParsed::Submit { text, keysym } => {
                                        if let Some(txt) = text {
                                            println!("txt {:?}", txt);
                                            // let keysym = xkbcommon::xkb::keysym_from_name(
                                            //     txt,
                                            //     KEYSYM_NO_FLAGS,
                                            // )
                                            // .raw()
                                            //     as i32;
                                            // virtual_keyboard_state.notify_keyboard_keysym(
                                            //     keysym,
                                            //     KeyState::Pressed.into(),
                                            // );
                                            // virtual_keyboard_state.notify_keyboard_keysym(
                                            //     keysym,
                                            //     KeyState::Released.into(),
                                            // );
                                        };
                                        if let Some(txt) = keysym {
                                            println!("txt {:?}", txt);
                                            // let keysym = xkbcommon::xkb::keysym_from_name(
                                            //     txt,
                                            //     KEYSYM_NO_FLAGS,
                                            // )
                                            // .raw()
                                            //     as i32;
                                            // virtual_keyboard_state.notify_keyboard_keysym(
                                            //     keysym,
                                            //     KeyState::Pressed.into(),
                                            // );
                                            // virtual_keyboard_state.notify_keyboard_keysym(
                                            //     keysym,
                                            //     KeyState::Released.into(),
                                            // );
                                        };
                                    }
                                    ActionParsed::Erase => {}
                                    ActionParsed::ShowPreferences => {}
                                    ActionParsed::Minimize => {}
                                    ActionParsed::Maximize => {}
                                }
                            }
                            this.key_pressed = button.cloned();
                            this.handle_key_press(button.cloned(), window, cx);
                            cx.notify();
                        }),
                    )
                    .on_mouse_up(
                        MouseButton::Left,
                        cx.listener(|this, _event, _window, cx| {
                            this.key_pressed = None;
                            cx.notify();
                        }),
                    )
                    .children(rows.into_iter().enumerate().map(|(i, (r_pos, row))| {
                        div()
                            .absolute()
                            .left(px(r_pos.x as f32))
                            .top(px(r_pos.y as f32))
                            .w(px(row.get_size().width as f32))
                            .h(px(row.get_size().height as f32))
                            .children(row.buttons.into_iter().enumerate().map(
                                |(j, (b_pos, button))| {
                                    let text = match button.label.clone() {
                                        Label::Text(t) => Some(t),
                                        _ => None,
                                    };

                                    let icon = match button.label {
                                        Label::Icon(i) => match i.as_str() {
                                            "key-enter" => Some(""),
                                            "keyboard-mode-symbolic" => Some(""),
                                            "edit-clear-symbolic" => {
                                                Some("icons/keyboard/backspace.svg")
                                            }
                                            "key-shift" => Some("icons/keyboard/shift.svg"),
                                            _ => None,
                                        },
                                        _ => None,
                                    };

                                    div()
                                        .absolute()
                                        .left(px(b_pos as f32))
                                        .top(px(0.0))
                                        .w(px(button.size.0 as f32))
                                        .h(px(button.size.1 as f32))
                                        .bg(colors.background_400)
                                        .border(px(1.))
                                        .border_color(colors.background_400)
                                        .rounded(px(4.))
                                        .when_some(self.key_pressed.clone(), |this, key_button| {
                                            if key_button.name == button.name {
                                                return this.bg(colors.background_300).when(
                                                    match key_button.label {
                                                        Label::Text(_) => true,
                                                        _ => false,
                                                    },
                                                    |this| {
                                                        this.child(
                                                            div()
                                                                .flex()
                                                                .items_center()
                                                                .justify_center()
                                                                .absolute()
                                                                .top(px(-button.size.1 as f32 - 8.))
                                                                .left(px(0 as f32))
                                                                .w(px(button.size.0 as f32))
                                                                .h(px(button.size.1 as f32))
                                                                .bg(colors.background_200)
                                                                .rounded(px(4.))
                                                                .text_size(px(22.))
                                                                .line_height(px(24.))
                                                                .font_weight(FontWeight(500.))
                                                                .text_color(colors.foreground_0)
                                                                .when_some(
                                                                    text.clone(),
                                                                    |this, text| this.child(text),
                                                                ),
                                                        )
                                                    },
                                                );
                                            }
                                            this
                                        })
                                        .items_center()
                                        .justify_center()
                                        .flex()
                                        .text_center()
                                        .text_size(px(22.))
                                        .line_height(px(24.))
                                        .font_weight(FontWeight(500.))
                                        .text_color(colors.foreground_0)
                                        .when_some(text, |this, text| this.child(text))
                                        .when_some(icon, |this, path| this.child(img(path)))
                                },
                            ))
                    })),
            )
    }
}
