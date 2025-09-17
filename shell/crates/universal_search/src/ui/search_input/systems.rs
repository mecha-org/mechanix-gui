use bevy::{
    color::palettes::css::*,
    input::keyboard::{Key, KeyboardInput},
    prelude::*,
    text::{self, cosmic_text::ttf_parser::apple_layout::state},
};

use crate::ui::search_input::{
    SearchActive, SearchText,
    components::{SearchClose, SearchInput, SearchInputText},
};

pub fn on_click(
    mut trigger: Trigger<Pointer<Click>>,
    q_search_input: Query<(), With<SearchInput>>,
    mut search_active: ResMut<SearchActive>,
    edit_text: Single<(&mut Text, &mut TextColor), (With<SearchInputText>)>,
) {
    if let Ok(()) = q_search_input.get(trigger.target()) {
        trigger.propagate(false);
        //send event to keyboard to open
        search_active.0 = true;
        let (mut text, mut text_color) = edit_text.into_inner();
        text.0 = "".to_string();
        text_color.0 = WHITE.into();
    }
}

pub fn on_close_click(
    mut trigger: Trigger<Pointer<Click>>,
    q_search_input: Query<(), With<SearchClose>>,
    mut search_active: ResMut<SearchActive>,
    mut search_text: ResMut<SearchText>,
    edit_text: Single<(&mut Text, &mut TextColor), (With<SearchInputText>)>,
) {
    if let Ok(()) = q_search_input.get(trigger.target()) {
        trigger.propagate(false);
        //send event to keyboard to close
        search_active.0 = true;
        search_text.0 = "".to_string();
        let (mut text, mut text_color) = edit_text.into_inner();
        text.0 = "Search here".to_string();
        text_color.0 = Color::oklch(0.7252, 0., 0.);
    }
}

pub fn listen_keyboard_input_events(
    mut commands: Commands,
    mut events: EventReader<KeyboardInput>,
    edit_text: Single<(&mut Text, &TextFont), (With<SearchInputText>)>,
    search_active: Res<SearchActive>,
    mut search_text: ResMut<SearchText>,
) {
    if !search_active.0 {
        return;
    }

    let (mut text, style) = edit_text.into_inner();
    for event in events.read() {
        // Only trigger changes when the key is first pressed.
        if !event.state.is_pressed() {
            continue;
        }

        match (&event.logical_key, event.key_code) {
            // (Key::Enter, _) => {
            //     if text.is_empty() {
            //         continue;
            //     }
            //     let old_value = mem::take(&mut **text);

            //     commands.spawn((
            //         Text2d::new(old_value),
            //         style.clone(),
            //         Bubble {
            //             timer: Timer::from_seconds(5.0, TimerMode::Once),
            //         },
            //     ));
            // }
            (_, KeyCode::Backspace) => {
                text.pop();
                search_text.0.pop();
            }
            (Key::Character(inserted_text), _) => {
                // Make sure the text doesn't have any control characters,
                // which can happen when keys like Escape are pressed
                if inserted_text.chars().all(is_printable_char) {
                    text.push_str(inserted_text);
                    search_text.0.push_str(inserted_text);
                }
            }
            _ => {
                continue;
            }
        }
    }
}

// this logic is taken from egui-winit:
// https://github.com/emilk/egui/blob/adfc0bebfc6be14cee2068dee758412a5e0648dc/crates/egui-winit/src/lib.rs#L1014-L1024
fn is_printable_char(chr: char) -> bool {
    let is_in_private_use_area = ('\u{e000}'..='\u{f8ff}').contains(&chr)
        || ('\u{f0000}'..='\u{ffffd}').contains(&chr)
        || ('\u{100000}'..='\u{10fffd}').contains(&chr);

    !is_in_private_use_area && !chr.is_ascii_control()
}
