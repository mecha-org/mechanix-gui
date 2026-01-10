use crate::types::{ActionParsed, FormattingError, KeyCode, KeySym};
use std::collections::HashMap;
use std::io::Write;
use std::{mem, ptr};

/// Index is the key code, String is the occupant.
type SingleKeyMap = [Option<String>; 256];

fn single_key_map_new() -> SingleKeyMap {
    let mut array = mem::MaybeUninit::<SingleKeyMap>::uninit();

    unsafe {
        let arref = &mut *array.as_mut_ptr();
        for element in arref.iter_mut() {
            ptr::write(element, None);
        }
        array.assume_init()
    }
}

/// Generates a mapping where each key gets a keycode, starting from 9
pub fn generate_keycodes<C: IntoIterator<Item = String>>(key_names: C) -> HashMap<String, KeyCode> {
    let mut sorted_names: Vec<String> = key_names.into_iter().collect();
    sorted_names.sort();

    HashMap::from_iter(sorted_names.into_iter().enumerate().map(|(idx, name)| {
        let keymap_idx = idx / 246; // 255 - 9 = 246 keys per keymap
        let code = 9 + (idx % 246) as u32;
        (name, KeyCode { code, keymap_idx })
    }))
}

pub fn generate_keymaps(
    symbolmap: HashMap<String, KeyCode>,
) -> Result<Vec<String>, FormattingError> {
    let mut bins: Vec<SingleKeyMap> = Vec::new();

    for (name, KeyCode { code, keymap_idx }) in symbolmap.into_iter() {
        if keymap_idx >= bins.len() {
            bins.resize_with(keymap_idx + 1, single_key_map_new);
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
        .enumerate()
        .filter_map(|(code, name)| name.as_ref().map(|n| (n, code)))
        .collect();

    for (_name, keycode) in &pairs {
        write!(buf, "\n        <I{}> = {0};", keycode)?;
    }

    writeln!(
        buf,
        "
        indicator 1 = \"Caps Lock\";
    }};
    
    xkb_symbols \"(unnamed)\" {{"
    )?;

    for (name, keycode) in pairs {
        write!(buf, "\nkey <I{}> {{ [ {} ] }};", keycode, name)?;
    }

    writeln!(
        buf,
        "
    }};

    xkb_types \"(unnamed)\" {{
        virtual_modifiers MechanixKeyboard;
    
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
        interpret Any+AnyOf(all) {{
            action= SetMods(modifiers=modMapMods,clearLocks);
        }};
    }};
}};"
    )?;

    String::from_utf8(buf).map_err(FormattingError::Utf)
}

pub fn extract_symbol_names<'a>(
    actions: &'a [(String, ActionParsed)],
) -> impl Iterator<Item = String> + 'a {
    actions
        .iter()
        .filter_map(|(_name, act)| match act {
            crate::types::ActionParsed::Submit { text: _, keysym } => Some(keysym.clone()),
            crate::types::ActionParsed::Erase => Some(Some("BackSpace".to_string())),
            _ => None,
        })
        .flatten()
        .map(|named_keysym| named_keysym)
}
