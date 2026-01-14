use crate::config::*;
use crate::keymap;
use crate::keymap::*;
use crate::layout::*;
use crate::types::*;
use crate::utils::*;
use anyhow::Result;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::io::BufReader;
use xkbcommon::xkb;

/// Main entry: build ParsedLayout from Layout
impl Layout {
    pub fn from_file(path: String) -> Result<Layout> {
        let infile = BufReader::new(fs::OpenOptions::new().read(true).open(&path)?);
        let layout: Layout = serde_yaml::from_reader(infile)?;
        Ok(layout)
    }

    pub fn build(self, keyboard_width: f64) -> Result<ParsedLayout> {
        // Step 1: Gather all unique button names (String keys)
        let button_names = self.gather_button_names();

        // Step 2: Map button names to their parsed Action
        let button_actions = self.create_button_actions(&button_names)?;

        // Step 3: Generate keycodes for all relevant key symbols
        let symbolmap = generate_keycodes(extract_symbol_names(&button_actions));

        // Step 4: Build all buttons' Key struct (with keycodes and actions)
        let button_states = self.create_button_states(&button_actions, &symbolmap);

        // Step 5: Generate keymaps for the whole symbolmap
        let keymaps = generate_keymaps(symbolmap)?;

        // Step 6: Build the complete set of views with button info
        let views = self.create_views(&button_states, keyboard_width);

        Ok(ParsedLayout { views, keymaps })
    }

    fn gather_button_names(&self) -> HashSet<String> {
        self.views
            .values()
            .flat_map(|rows| {
                rows.iter()
                    .flat_map(|row| row.split_ascii_whitespace().map(|name| name.to_string()))
            })
            .collect()
    }

    fn create_button_actions(
        &self,
        button_names: &HashSet<String>,
    ) -> Result<Vec<(String, ActionParsed)>> {
        let view_names: Vec<String> = self.views.keys().cloned().collect();
        Ok(button_names
            .iter()
            .map(|name| {
                (
                    name.clone(),
                    ActionBuilder::create_action(&self.buttons, name, &view_names),
                )
            })
            .collect())
    }

    fn create_button_states(
        &self,
        button_actions: &[(String, ActionParsed)],
        symbolmap: &HashMap<String, KeyCode>,
    ) -> HashMap<String, Key> {
        button_actions
            .iter()
            .map(|(name, action)| {
                let keycodes = KeyCodeExtractor::extract_keycodes(&action, symbolmap);
                (
                    name.clone(),
                    Key {
                        keycodes,
                        action: action.clone(),
                    },
                )
            })
            .collect()
    }

    pub fn create_views(
        &self,
        button_states: &HashMap<String, Key>,
        keyboard_width: f64,
    ) -> HashMap<String, View> {
        let mut views_map = HashMap::new();

        for (view_name, rows) in &self.views {
            // First, build vector of rows of KeyButtons (without offsets)
            let rows_buttons: Vec<Vec<KeyButton>> = rows
                .iter()
                .map(|row_str| {
                    row_str
                        .split_ascii_whitespace()
                        .map(|btn_name| {
                            ButtonBuilder::create_keyboard_button(
                                btn_name,
                                &self.buttons,
                                &self.outlines,
                                button_states
                                    .get(btn_name)
                                    .expect("Button state not created")
                                    .clone(),
                            )
                        })
                        .collect()
                })
                .collect();

            // Calculate widths of each row (sum of buttons width + spacing)
            let gap_cfg = &self.gap.col;
            let row_widths: Vec<f64> = rows_buttons
                .iter()
                .map(|button_row| {
                    let mut width = 0.0;
                    for (i, btn) in button_row.iter().enumerate() {
                        width += btn.size.0;
                        // Add custom gap after button if not last button
                        if i < button_row.len() - 1 {
                            let gap = gap_cfg
                                .custom
                                .get(btn.name.as_str())
                                .copied()
                                .unwrap_or(gap_cfg.default);
                            width += gap;
                        }
                    }
                    width
                })
                .collect();

            // Find max row width
            let max_width = keyboard_width;

            // Now build rows with horizontal offsets, centered based on max_width
            let row_objs: Vec<(f64, Row)> = rows_buttons
                .into_iter()
                .zip(row_widths.into_iter())
                .map(|(button_row, row_width)| {
                    let start_offset = (max_width - row_width) / 2.0;
                    println!(
                        "Max width: {:.2}, Row width: {:.2}, start offset: {:.2}",
                        max_width, row_width, start_offset
                    );

                    let buttons_with_offsets: Vec<(f64, KeyButton)> = add_offsets(
                        button_row.into_iter(),
                        |button| button.name.as_str(),
                        |button| button.size.0,
                        gap_cfg,
                        start_offset,
                    )
                    .collect();

                    (0.0, Row::new(buttons_with_offsets))
                    // The vertical offset (y) will be assigned later
                })
                .collect();

            // Add vertical offsets (rows stacked vertically)
            let row_gap_cfg = &self.gap.row;
            let rows_w_vertical_offsets: Vec<(f64, Row)> = {
                let mut offset = 0.0;
                row_objs
                    .into_iter()
                    .map(|(_x, row)| {
                        let y_offset = offset;
                        offset += row.get_size().height + row_gap_cfg.default;
                        (y_offset, row)
                    })
                    .collect()
            };

            // Construct View with all rows positioned
            let view = View::new(rows_w_vertical_offsets);

            views_map.insert(view_name.clone(), view);
        }

        views_map
    }
}

/// Action construction helpers
struct ActionBuilder;

impl ActionBuilder {
    fn create_action(
        button_info: &HashMap<String, ButtonMeta>,
        name: &str,
        view_names: &[String],
    ) -> ActionParsed {
        let default_meta = ButtonMeta::default();
        let symbol_meta = button_info.get(name).unwrap_or(&default_meta);

        let submission = match (
            &symbol_meta.action,
            &symbol_meta.keysym,
            &symbol_meta.text,
            &symbol_meta.modifier,
        ) {
            (Some(action), None, None, None) => SubmitData::Action(action.clone()),
            (None, Some(keysym), None, None) => SubmitData::Keysym(keysym.clone()),
            (None, None, Some(text), None) => SubmitData::Text(text.clone()),
            (None, None, None, Some(modifier)) => SubmitData::Modifier(modifier.clone()),
            (None, None, None, None) => SubmitData::Text(name.into()),
            _ => {
                eprintln!(
                    "Button {} has more than one of (action, keysym, text, modifier)",
                    name
                );
                SubmitData::Text("".into())
            }
        };

        fn filter_view_name(button_name: &str, view_name: String, view_names: &[String]) -> String {
            if view_names.contains(&view_name) {
                view_name
            } else {
                eprintln!(
                    "Button {} switches to missing view {}",
                    button_name, view_name
                );
                "base".into()
            }
        }

        match submission {
            SubmitData::Action(Action::SetView(view_name)) => {
                ActionParsed::SetView(filter_view_name(name, view_name, view_names))
            }
            SubmitData::Action(Action::Locking {
                lock_view,
                unlock_view,
                pops,
                looks_locked_from,
            }) => ActionParsed::LockView {
                lock: filter_view_name(name, lock_view, view_names),
                unlock: filter_view_name(name, unlock_view, view_names),
                latches: pops.unwrap_or(true),
                looks_locked_from,
            },
            SubmitData::Action(Action::ShowPrefs) => ActionParsed::ShowPreferences,
            SubmitData::Action(Action::Minimize) => ActionParsed::Minimize,
            SubmitData::Action(Action::Maximize) => ActionParsed::Maximize,
            SubmitData::Action(Action::Erase) => ActionParsed::Erase,
            SubmitData::Keysym(keysym) => ActionParsed::Submit {
                text: None,
                keysym: Some(keysym.clone()),
            },
            SubmitData::Text(text) => ActionParsed::Submit {
                text: Some(text.clone()),
                keysym: None,
            },
            SubmitData::Modifier(modifier) => match modifier {
                Modifier::Control => ActionParsed::ApplyModifier(ModifierParsed::Control),
                Modifier::Alt => ActionParsed::ApplyModifier(ModifierParsed::Alt),
                Modifier::Mod4 => ActionParsed::ApplyModifier(ModifierParsed::Mod4),
                unsupported_modifier => {
                    eprintln!("Modifier {:?} unsupported", unsupported_modifier);
                    ActionParsed::Submit {
                        text: None,
                        keysym: None,
                    }
                }
            },
        }
    }
}

#[derive(Clone)]
enum SubmitData {
    Action(Action),
    Text(String),
    Keysym(String),
    Modifier(Modifier),
}

fn keysym_valid(name: &str) -> bool {
    xkb::keysym_from_name(name, xkb::KEYSYM_NO_FLAGS) != xkb::keysyms::KEY_NoSymbol.into()
}

struct KeyCodeExtractor;

impl KeyCodeExtractor {
    fn extract_keycodes(
        action: &ActionParsed,
        symbolmap: &HashMap<String, KeyCode>,
    ) -> Vec<KeyCode> {
        match action {
            ActionParsed::Submit { keysym, .. } => keysym
                .iter()
                .filter_map(|named_keysym| symbolmap.get(named_keysym).cloned())
                .collect(),
            ActionParsed::Erase => symbolmap
                .get("BackSpace")
                .map(|keycode| vec![keycode.clone()])
                .unwrap_or_default(),
            _ => Vec::new(),
        }
    }
}

struct ButtonBuilder;

impl ButtonBuilder {
    fn create_keyboard_button(
        name: &str,
        button_info: &HashMap<String, ButtonMeta>,
        outlines: &HashMap<String, Outline>,
        data: Key,
    ) -> KeyButton {
        let name = name.to_string();
        let default_meta = ButtonMeta::default();
        let button_meta = button_info.get(&name).unwrap_or(&default_meta);

        let label = if let Some(label) = &button_meta.label {
            Label::Text(label.clone())
        } else if let Some(icon) = &button_meta.icon {
            Label::Icon(icon.clone())
        } else if let Some(text) = &button_meta.text {
            Label::Text(text.clone())
        } else {
            Label::Text(name.clone())
        };

        let outline_name = match &button_meta.outline {
            Some(outline) if outlines.contains_key(outline) => outline.clone(),
            Some(outline) => {
                eprintln!(
                    "Outline named {} does not exist! Using default for button {}",
                    outline, name
                );
                "default".into()
            }
            None => "default".into(),
        };

        let outline = outlines.get(&outline_name).cloned().unwrap_or(Outline {
            width: 1.0,
            height: 1.0,
        });

        KeyButton {
            name,
            outline_name,
            size: (outline.width, outline.height),
            label,
            action: data.action,
            keycodes: data.keycodes,
        }
    }
}
