use gpui::{input_method::KeyState, prelude::FluentBuilder, *};
use icons::prelude::Icons;
use settings::prelude::Settings;
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
    pub _poll_task: Task<()>,
    pub prev_surrounding_text: Option<(String, u32, u32)>,
    pub was_active: bool,
}

impl OnScreenKeyboard {
    pub fn new(parsed_layout: ParsedLayout, trie: Trie, cx: &mut Context<Self>) -> Self {
        let _poll_task = cx.spawn(
            async move |this: WeakEntity<Self>, cx: &mut AsyncApp| loop {
                let executor = cx.background_executor().clone();
                cx.background_spawn(async move {
                    executor.timer(std::time::Duration::from_millis(100)).await;
                })
                .await;

                let _ = this.update(cx, |_this, cx| {
                    cx.notify();
                });
            },
        );

        Self {
            current_view: "base".to_string(),
            current_layout: parsed_layout,
            key_pressed: None,
            suggestions: Vec::from([]),
            suggested_for: String::new(),
            trie,
            _poll_task,
            prev_surrounding_text: None,
            was_active: false,
        }
    }

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
                        if let Some(text) = text {
                            im.commit_string(&text);
                            im.commit();
                        }
                        if let Some(keysym) = keysym {
                            match keysym.as_str() {
                                "space" => {
                                    im.commit_string(" ");
                                    im.commit();
                                }
                                "Return" => {
                                    if let Some(vk) = window.get_virtual_keyboard() {
                                        let timestamp = std::time::SystemTime::now()
                                            .duration_since(std::time::UNIX_EPOCH)
                                            .unwrap()
                                            .as_millis()
                                            as u32;
                                        vk.send_key(timestamp, 28, KeyState::Pressed);
                                        vk.send_key(timestamp, 28, KeyState::Released);
                                    }
                                }
                                "BackSpace" => {
                                    // im.delete_surrounding_text(1, 0);
                                    // im.commit();
                                    if let Some(vk) = window.get_virtual_keyboard() {
                                        let timestamp = std::time::SystemTime::now()
                                            .duration_since(std::time::UNIX_EPOCH)
                                            .unwrap()
                                            .as_millis()
                                            as u32;
                                        vk.send_key(timestamp, 14, KeyState::Pressed);
                                        vk.send_key(timestamp, 14, KeyState::Released);
                                    }
                                }
                                _ => (),
                            }
                        }
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
        cx: &mut Context<Self>,
    ) {
        if let Some(im) = window.get_input_method() {
            im.delete_surrounding_text(self.suggested_for.len() as u32, 0);
            im.commit_string(suggestion);
            im.commit();
            cx.notify();
        }
    }
}

impl Render for OnScreenKeyboard {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // Show keyboard when input method is active (text field has focus)
        let is_active = window.is_input_method_active();
        let settings = Settings::global(cx).keyboard.clone().layer_shell.size;

        if is_active != self.was_active {
            self.was_active = is_active;
            if is_active {
                window.resize(size(settings.width, settings.height));
            } else {
                window.resize(size(px(1.), px(1.)));
            }

            cx.notify();
        }

        if let Some(im) = window.get_input_method() {
            if self.prev_surrounding_text != im.get_surrounding_text() {
                self.prev_surrounding_text = im.get_surrounding_text();
                if let Some((text, cursor, _anchor)) = im.get_surrounding_text() {
                    let words = &text.as_str()[0..cursor as usize].split(" ");
                    if let Some(last) = words.clone().last() {
                        let suggestions = self.trie.search(last);
                        // let next_char_prob = self.trie.next_char_probabilities(last);
                        self.suggestions = suggestions;
                        self.suggested_for = last.to_ascii_lowercase();
                        // Self::get().next_char_prob.set(next_char_prob);
                    }
                };
                cx.notify();
            };
        }

        let view = self
            .current_layout
            .get_view(self.current_view.as_str())
            .unwrap();
        let rows = view.rows.clone();
        let colors = cx.theme().colors.clone();
        let suggestions = self.suggestions.clone();
        let icons = Icons::global(cx).keyboard.clone();

        div()
            .size_full()
            .flex()
            .flex_col()
            .child(
                div()
                    .w_full()
                    .h(px(48.))
                    .pl(px(12.))
                    .pr(px(12.))
                    .pt(px(8.))
                    .pb(px(5.))
                    .bg(colors.background_900)
                    .flex()
                    .flex_row()
                    .items_center()
                    .justify_between()
                    .children(suggestions.iter().enumerate().map(|(i, s)| {
                        div()
                            .id(("suggestion", i))
                            .size_full()
                            .flex()
                            .flex_row()
                            .items_center()
                            .justify_center()
                            .text_color(colors.foreground_800)
                            .text_size(px(18.))
                            .font_weight(FontWeight(400.))
                            .when(i != &suggestions.iter().len() - 1, |this| {
                                this.border_r_1().border_color(colors.background_600)
                            })
                            .child(s.clone())
                            .on_click({
                                let s = s.clone();
                                cx.listener(move |this, _event, window, cx| {
                                    this.handle_suggestion_press(&s, window, cx);
                                })
                            })
                    })),
            )
            .child(
                div()
                    .id("click-area")
                    .w_full()
                    .h(settings.height - px(48.))
                    .pt(px(6.))
                    .bg(colors.background_900)
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
                                event.position.y.to_f64() - 48.,
                            );
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
                            cx.notify();
                        }),
                    )
                    .on_mouse_up(
                        MouseButton::Left,
                        cx.listener(|this, _event, window, cx| {
                            this.handle_key_press(this.key_pressed.clone(), window, cx);
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
                                            "key-enter" => None,
                                            "keyboard-mode-symbolic" => None,
                                            "edit-clear-symbolic" => Some(icons.backspace.clone()),
                                            "key-shift" => Some(icons.shift.clone()),
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
                                        .bg(colors.background_600)
                                        .border(px(1.))
                                        .border_color(colors.background_600)
                                        .rounded(px(4.))
                                        .when_some(self.key_pressed.clone(), |this, key_button| {
                                            let is_text_key = match key_button.action {
                                                ActionParsed::Submit { text, keysym } => {
                                                    text.is_some()
                                                }
                                                _ => false,
                                            };
                                            if key_button.name == button.name {
                                                return this.bg(colors.background_300).when(
                                                    is_text_key,
                                                    |this| {
                                                        this.child(
                                                            div()
                                                                .flex()
                                                                .items_center()
                                                                .justify_center()
                                                                .absolute()
                                                                .top(px(-button.size.1 as f32 - 1.))
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
                                        .text_size(
                                            if matches!(button.action, ActionParsed::SetView(..)) {
                                                px(18.)
                                            } else {
                                                px(22.)
                                            },
                                        )
                                        .line_height(px(24.))
                                        .font_weight(FontWeight(500.))
                                        .text_color(colors.foreground_100)
                                        .when_some(text, |this, text| this.child(text))
                                        .when_some(icon, |this, path| this.child(img(path)))
                                },
                            ))
                    })),
            )
    }
}
