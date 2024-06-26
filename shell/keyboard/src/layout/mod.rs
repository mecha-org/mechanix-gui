use anyhow::Result;
use serde::Deserialize;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::io::BufReader;
use std::path::PathBuf;
use std::vec::Vec;

use crate::errors::KeyboardError;

/// The root element describing an entire keyboard
#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(deny_unknown_fields)]
pub struct Layout {
    #[serde(default)]
    pub margins: Margins,
    pub views: HashMap<String, Vec<ButtonIds>>,
    #[serde(default)]
    pub buttons: HashMap<String, ButtonMeta>,
    pub outlines: HashMap<String, Outline>,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Default)]
#[serde(deny_unknown_fields)]
struct Margins {
    top: f64,
    bottom: f64,
    side: f64,
}

/// Buttons are embedded in a single string
type ButtonIds = String;

/// All info about a single button
/// Buttons can have multiple instances though.
#[derive(Debug, Default, Deserialize, PartialEq, Clone)]
#[serde(deny_unknown_fields)]
struct ButtonMeta {
    // TODO: structure (action, keysym, text, modifier) as an enum
    // to detect conflicts and missing values at compile time
    /// Special action to perform on activation.
    /// Conflicts with keysym, text, modifier.
    #[serde(with = "serde_yaml::with::singleton_map", default)]
    action: Option<Action>,
    /// The name of the XKB keysym to emit on activation.
    /// Conflicts with action, text, modifier.
    keysym: Option<String>,
    /// The text to submit on activation. Will be derived from ID if not present
    /// Conflicts with action, keysym, modifier.
    text: Option<String>,
    /// The modifier to apply while the key is locked
    /// Conflicts with action, keysym, text
    modifier: Option<Modifier>,
    /// If not present, will be derived from text or the button ID
    label: Option<String>,
    /// Conflicts with label
    icon: Option<String>,
    /// The name of the outline. If not present, will be "default"
    outline: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(deny_unknown_fields)]
enum Action {
    #[serde(rename = "locking")]
    Locking {
        lock_view: String,
        unlock_view: String,
        pops: Option<bool>,
        #[serde(default)]
        looks_locked_from: Vec<String>,
    },
    #[serde(rename = "set_view")]
    SetView(String),
    #[serde(rename = "show_prefs")]
    ShowPrefs,
    /// Remove last character
    #[serde(rename = "erase")]
    Erase,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
enum Modifier {
    Control,
    Shift,
    Lock,
    #[serde(alias = "Mod1")]
    Alt,
    Mod2,
    Mod3,
    Mod4,
    Mod5,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
struct Outline {
    width: f64,
    height: f64,
}

pub fn add_offsets<'a, I: 'a, T, F: 'a>(
    iterator: I,
    get_size: F,
) -> impl Iterator<Item = (f64, T)> + 'a
where
    I: Iterator<Item = T>,
    F: Fn(&T) -> f64,
{
    let mut offset = 0.0;
    iterator.map(move |item| {
        let size = get_size(&item);
        let value = (offset, item);
        offset += size;
        value
    })
}

impl Layout {
    pub fn from_file(path: String) -> Result<Layout> {
        let infile = BufReader::new(fs::OpenOptions::new().read(true).open(&path)?);
        let layout: Layout = serde_yaml::from_reader(infile)?;
        Ok(layout)
    }

    pub fn build(mut self) -> Result<ParsedLayout> {
        let button_names = self
            .views
            .values()
            .flat_map(|rows| rows.iter().flat_map(|row| row.split_ascii_whitespace()));
        let button_names: HashSet<&str> = HashSet::from_iter(button_names);

        let button_actions: Vec<(&str, crate::action::Action)> = button_names
            .iter()
            .map(|name| {
                (
                    *name,
                    create_action(&self.buttons, name, self.views.keys().collect()),
                )
            })
            .collect();

        let button_states = HashMap::<String, crate::action::Action>::from_iter(
            button_actions.into_iter().map(|(name, action)| {
                // let keycodes = match &action {
                //     // crate::action::Action::Submit { text: _, keys } => keys
                //     //     .iter()
                //     //     .map(|named_keysym| {
                //     //         symbolmap
                //     //             .get(named_keysym.0.as_str())
                //     //             .expect(
                //     //                 format!(
                //     //                     "keysym {} in key {} missing from symbol map",
                //     //                     named_keysym.0, name
                //     //                 )
                //     //                 .as_str(),
                //     //             )
                //     //             .clone()
                //     //     })
                //     //     .collect(),
                //     crate::action::Action::Erase => vec![symbolmap
                //         .get("BackSpace")
                //         .expect(&format!("BackSpace missing from symbol map"))
                //         .clone()],
                //     _ => Vec::new(),
                // };
                (name.into(), action)
            }),
        );

        let views: Vec<_> = self
            .views
            .iter()
            .map(|(name, view)| {
                let rows = view.iter().map(|row| {
                    let buttons = row.split_ascii_whitespace().map(|name| {
                        create_keyboard_button(
                            name,
                            &self.buttons,
                            &self.outlines,
                            button_states
                                .get(name.into())
                                .expect("Button state not created")
                                .clone(),
                        )
                    });
                    Row::new(buttons.collect())
                });
                (name.clone(), View::new(rows.collect()))
            })
            .collect();

        // Center views on the same point.
        let views: HashMap<String, View> =
            { HashMap::from_iter(views.into_iter().map(|(name, view)| (name, view))) };

        Ok(ParsedLayout { views })
    }
}

#[derive(Debug, Clone)]
pub enum Label {
    Text(String),
    Icon(String),
}

#[derive(Debug, Clone)]
pub struct KeyButton {
    /// ID
    pub name: String,
    pub label: Label,
    pub size: (f64, f64),
    /// The name of the visual class applied
    pub outline_name: String,
    /// Static description of what the key does when pressed or released
    pub action: crate::action::Action,
}

fn create_keyboard_button(
    name: &str,
    button_info: &HashMap<String, ButtonMeta>,
    outlines: &HashMap<String, Outline>,
    data: crate::action::Action,
) -> KeyButton {
    let name = name.to_string();
    // don't remove, because multiple buttons with the same name are allowed
    let default_meta = ButtonMeta::default();
    let button_meta = button_info.get(&name).unwrap_or(&default_meta);

    // TODO: move conversion to the C/Rust boundary
    let label = if let Some(label) = &button_meta.label {
        crate::layout::Label::Text(String::from(label.as_str()))
    } else if let Some(icon) = &button_meta.icon {
        crate::layout::Label::Icon(String::from(icon.as_str()))
    } else if let Some(text) = &button_meta.text {
        crate::layout::Label::Text(String::from(text.as_str()))
    } else {
        crate::layout::Label::Text(name.clone())
    };

    let outline_name = match &button_meta.outline {
        Some(outline) => {
            if outlines.contains_key(outline) {
                outline.clone()
            } else {
                println!(
                    "{:?}",
                    format!(
                        "Outline named {} does not exist! Using default for button {}",
                        outline, name
                    )
                );
                "default".into()
            }
        }
        None => "default".into(),
    };

    //Check for no default outline defined Using 1x1
    let outline = outlines
        .get(&outline_name)
        .map(|outline| (*outline).clone())
        .unwrap_or(Outline {
            width: 1f64,
            height: 1f64,
        });

    KeyButton {
        name,
        outline_name,
        size: (outline.width, outline.height),
        label,
        action: data,
    }
}

fn create_action(
    button_info: &HashMap<String, ButtonMeta>,
    name: &str,
    view_names: Vec<&String>,
) -> crate::action::Action {
    let default_meta = ButtonMeta::default();
    let symbol_meta = button_info.get(name).unwrap_or(&default_meta);

    fn keysym_valid(name: &str) -> bool {
        true
        // xkb::keysym_from_name(name, xkb::KEYSYM_NO_FLAGS) != xkb::KEY_NoSymbol
    }

    enum SubmitData {
        Action(Action),
        Text(String),
        Keysym(String),
        Modifier(Modifier),
    }

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
            println!(
                "{:?}",
                format!(
                    "Button {} has more than one of (action, keysym, text, modifier)",
                    name,
                )
            );
            SubmitData::Text("".into())
        }
    };

    fn filter_view_name(button_name: &str, view_name: String, view_names: &Vec<&String>) -> String {
        if view_names.contains(&&view_name) {
            view_name
        } else {
            println!(
                "{:?}",
                format!(
                    "Button {} switches to missing view {}",
                    button_name, view_name,
                )
            );
            "base".into()
        }
    }

    match submission {
        SubmitData::Action(Action::SetView(view_name)) => {
            crate::action::Action::SetView(filter_view_name(name, view_name.clone(), &view_names))
        }
        SubmitData::Action(Action::Locking {
            lock_view,
            unlock_view,
            pops,
            looks_locked_from,
        }) => crate::action::Action::LockView {
            lock: filter_view_name(name, lock_view.clone(), &view_names),
            unlock: filter_view_name(name, unlock_view.clone(), &view_names),
            latches: pops.unwrap_or(true),
            looks_locked_from,
        },
        SubmitData::Action(Action::ShowPrefs) => crate::action::Action::ShowPreferences,
        SubmitData::Action(Action::Erase) => crate::action::Action::Erase,
        SubmitData::Keysym(keysym) => crate::action::Action::Submit {
            text: None,
            keys: vec![crate::action::KeySym(match keysym_valid(keysym.as_str()) {
                true => keysym.clone(),
                false => {
                    println!("{:?}", format!("Keysym name invalid: {}", keysym,));
                    "space".into() // placeholder
                }
            })],
        },
        SubmitData::Text(text) => crate::action::Action::Submit {
            text: Some(String::from(text.clone())),
            keys: text
                .chars()
                .map(|codepoint| {
                    let codepoint_string = codepoint.to_string();
                    crate::action::KeySym(match keysym_valid(codepoint_string.as_str()) {
                        true => codepoint_string,
                        false => format!("U{:04X}", codepoint as u32),
                    })
                })
                .collect(),
        },
        SubmitData::Modifier(modifier) => match modifier {
            Modifier::Control => {
                crate::action::Action::ApplyModifier(crate::action::Modifier::Control)
            }
            Modifier::Alt => crate::action::Action::ApplyModifier(crate::action::Modifier::Alt),
            Modifier::Mod4 => crate::action::Action::ApplyModifier(crate::action::Modifier::Mod4),
            unsupported_modifier => {
                println!(
                    "{:?}",
                    format!("Modifier {:?} unsupported", unsupported_modifier,)
                );
                crate::action::Action::Submit {
                    text: None,
                    keys: Vec::new(),
                }
            }
        },
    }
}

fn extract_symbol_names<'a>(
    actions: &'a [(&str, crate::action::Action)],
) -> impl Iterator<Item = String> + 'a {
    actions
        .iter()
        .filter_map(|(_name, act)| match act {
            crate::action::Action::Submit { text: _, keys } => Some(keys.clone()),
            crate::action::Action::Erase => Some(vec![crate::action::KeySym("BackSpace".into())]),
            _ => None,
        })
        .flatten()
        .map(|named_keysym| named_keysym.0)
}

#[derive(Debug, Clone)]
pub struct Row {
    pub buttons: Vec<KeyButton>,
}

impl Row {
    pub fn new(buttons: Vec<KeyButton>) -> Row {
        Row { buttons }
    }
}

#[derive(Debug, Clone)]
pub struct View {
    pub rows: Vec<Row>,
}

impl View {
    pub fn new(rows: Vec<Row>) -> View {
        View { rows }
    }
}

#[derive(Debug, Clone)]
pub struct ParsedLayout {
    pub views: HashMap<String, View>,
}
