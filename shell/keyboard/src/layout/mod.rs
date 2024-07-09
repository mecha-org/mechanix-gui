use anyhow::Result;
use serde::Deserialize;
use std::collections::{HashMap, HashSet};
use std::io::BufReader;
use std::io::Write;
use std::mem;
use std::ptr;
use std::string::FromUtf8Error;
use std::vec::Vec;
use std::{fmt, fs, io};
use xkbcommon::xkb;
mod util;

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

        let symbolmap: HashMap<String, KeyCode> =
            generate_keycodes(extract_symbol_names(&button_actions));

        // println!("symbolmap {:?}", symbolmap);

        let button_states =
            HashMap::<String, Key>::from_iter(button_actions.into_iter().map(|(name, action)| {
                let keycodes = match &action {
                    crate::action::Action::Submit { text: _, keys } => keys
                        .iter()
                        .map(|named_keysym| {
                            symbolmap
                                .get(named_keysym.0.as_str())
                                .expect(
                                    format!(
                                        "keysym {} in key {} missing from symbol map",
                                        named_keysym.0, name
                                    )
                                    .as_str(),
                                )
                                .clone()
                        })
                        .collect(),
                    crate::action::Action::Erase => vec![symbolmap
                        .get("BackSpace")
                        .expect(&format!("BackSpace missing from symbol map"))
                        .clone()],
                    _ => Vec::new(),
                };
                (name.into(), Key { keycodes, action })
            }));

        let keymaps = match generate_keymaps(symbolmap) {
            Err(e) => return (Err(anyhow::Error::msg(e.to_string()))),
            Ok(v) => v,
        };

        // println!("keymaps {:?}", keymaps);
        // println!("keymaps len {:?}", keymaps.len());

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

        Ok(ParsedLayout { views, keymaps })
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
    pub keycodes: Vec<crate::layout::KeyCode>,
}

fn create_keyboard_button(
    name: &str,
    button_info: &HashMap<String, ButtonMeta>,
    outlines: &HashMap<String, Outline>,
    data: Key,
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
        action: data.action,
        keycodes: data.keycodes,
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
        xkb::keysym_from_name(name, xkb::KEYSYM_NO_FLAGS) != xkb::KEY_NoSymbol
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
    pub keymaps: Vec<String>,
}

/// The extended, unambiguous layout-keycode
#[derive(Debug, Clone, PartialEq)]
pub struct KeyCode {
    pub code: u32,
    pub keymap_idx: usize,
}

/// Generates a mapping where each key gets a keycode, starting from ~~8~~
/// HACK: starting from 9, because 8 results in keycode 0,
/// which the compositor likes to discard
pub fn generate_keycodes<'a, C: IntoIterator<Item = String>>(
    key_names: C,
) -> HashMap<String, KeyCode> {
    HashMap::from_iter(
        // Sort to remove a source of indeterminism in keycode assignment.
        sorted(key_names.into_iter())
            .zip(util::cycle_count(9..255))
            .map(|(name, (code, keymap_idx))| (String::from(name), KeyCode { code, keymap_idx })),
    )
}

/// Sorts an iterator by converting it to a Vector and back
fn sorted<'a, I: Iterator<Item = String>>(iter: I) -> impl Iterator<Item = String> {
    let mut v: Vec<String> = iter.collect();
    v.sort();
    v.into_iter()
}

#[derive(Debug)]
pub enum FormattingError {
    Utf(FromUtf8Error),
    Format(io::Error),
}

impl fmt::Display for FormattingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FormattingError::Utf(e) => write!(f, "UTF: {}", e),
            FormattingError::Format(e) => write!(f, "Format: {}", e),
        }
    }
}

impl From<io::Error> for FormattingError {
    fn from(e: io::Error) -> Self {
        FormattingError::Format(e)
    }
}

#[derive(Debug, Clone)]
pub struct Key {
    /// A cache of raw keycodes derived from Action::Submit given a keymap
    pub keycodes: Vec<KeyCode>,
    /// Static description of what the key does when pressed or released
    pub action: crate::action::Action,
}

/// Index is the key code, String is the occupant.
/// Starts all empty.
/// https://gitlab.freedesktop.org/xorg/xserver/-/issues/260
type SingleKeyMap = [Option<String>; 256];

fn single_key_map_new() -> SingleKeyMap {
    // Why can't we just initialize arrays without tricks -_- ?
    // Inspired by
    // https://www.reddit.com/r/rust/comments/5n7bh1/how_to_create_an_array_of_a_type_with_clone_but/
    let mut array = mem::MaybeUninit::<SingleKeyMap>::uninit();

    unsafe {
        let arref = &mut *array.as_mut_ptr();
        for element in arref.iter_mut() {
            ptr::write(element, None);
        }

        array.assume_init()
    }
}

pub fn generate_keymaps(
    symbolmap: HashMap<String, KeyCode>,
) -> Result<Vec<String>, FormattingError> {
    let mut bins: Vec<SingleKeyMap> = Vec::new();

    for (name, KeyCode { code, keymap_idx }) in symbolmap.into_iter() {
        if keymap_idx >= bins.len() {
            bins.resize_with(keymap_idx + 1, || single_key_map_new());
        }
        bins[keymap_idx][code as usize] = Some(name);
    }

    let mut out = Vec::new();
    for bin in bins {
        out.push(generate_keymap(&bin)?);
    }
    Ok(out)
}

/// Generates a de-facto single level keymap.
/// Key codes must not repeat and must remain between 9 and 255.
fn generate_keymap(symbolmap: &SingleKeyMap) -> Result<String, FormattingError> {
    let mut buf: Vec<u8> = Vec::new();
    writeln!(
        buf,
        "xkb_keymap {{

    xkb_keycodes \"(unnamed)\" {{
        minimum = 8;
        maximum = 255;"
    )?;

    let pairs: Vec<(&String, usize)> = symbolmap
        .iter()
        // Attach a key code to each cell.
        .enumerate()
        // Get rid of empty keycodes.
        .filter_map(|(code, name)| name.as_ref().map(|n| (n, code)))
        .collect();

    // Xorg can only consume up to 255 keys, so this may not work in Xwayland.
    // Two possible solutions:
    // - use levels to cram multiple characters into one key
    // - swap layouts on key presses
    for (_name, keycode) in &pairs {
        write!(
            buf,
            "
        <I{}> = {0};",
            keycode,
        )?;
    }

    writeln!(
        buf,
        "
        indicator 1 = \"Caps Lock\"; // Xwayland won't accept without it.
    }};
    
    xkb_symbols \"(unnamed)\" {{
"
    )?;

    for (name, keycode) in pairs {
        write!(
            buf,
            "
key <I{}> {{ [ {} ] }};",
            keycode, name,
        )?;
    }

    writeln!(
        buf,
        "
    }};

    xkb_types \"(unnamed)\" {{
        virtual_modifiers MechanixKeyboard; // No modifiers! Needed for Xorg for some reason.
    
        // Those names are needed for Xwayland.
        type \"ONE_LEVEL\" {{
            modifiers= none;
            level_name[Level1]= \"Any\";
        }};
        type \"TWO_LEVEL\" {{
            level_name[Level1]= \"Base\";
        }};
        type \"ALPHABETIC\" {{
            level_name[Level1]= \"Base\";
        }};
        type \"KEYPAD\" {{
            level_name[Level1]= \"Base\";
        }};
        type \"SHIFT+ALT\" {{
            level_name[Level1]= \"Base\";
        }};

    }};

    xkb_compatibility \"(unnamed)\" {{
        // Needed for Xwayland again.
        interpret Any+AnyOf(all) {{
            action= SetMods(modifiers=modMapMods,clearLocks);
        }};
    }};
}};"
    )?;

    String::from_utf8(buf).map_err(FormattingError::Utf)
}
