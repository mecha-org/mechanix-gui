use configparser::ini::Ini;
use log::debug;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::path::Path;
use std::time::UNIX_EPOCH;

#[derive(Debug, Default, Clone)]
pub struct DesktopEntry {
    pub type_: String,
    pub name: String,
    pub generic_name: Option<String>,
    pub comment: Option<String>,
    pub exec: Option<String>,
    pub try_exec: Option<String>,
    pub icon: Option<String>,
    pub categories: Vec<String>,
    pub mime_type: Vec<String>,
    pub keywords: Vec<String>,
    pub actions: Vec<String>,
    pub terminal: bool,
    pub no_display: bool,
    pub hidden: bool,
    pub other_keys: HashMap<String, String>,
}

pub fn parse_desktop_entry(path: &Path) -> Option<DesktopEntry> {
    // let raw = fs::read_to_string(path).ok()?;
    // debug!("Raw string of desktop entry: {}", raw);
    let mut config = Ini::new();
    let config = match config.load(path) {
        Ok(c) => c,
        Err(e) => {
            debug!("Failed to parse desktop entry: {}", e);
            return None;
        }
    };

    let ini = match config.get("desktop entry") {
        Some(i) => i,
        None => return None,
    };

    let get_str = |key: &str| -> Option<String> {
        if key.is_empty() {
            return None;
        }
        ini.get(key)?.as_ref().map(|v| v.trim().to_string())
    };

    let get_bool = |key: &str| -> bool {
        if key.is_empty() {
            return false;
        }
        get_str(key)
            .map(|v| v.eq_ignore_ascii_case("true"))
            .unwrap_or(false)
    };

    let get_list = |key: &str| -> Vec<String> {
        if key.is_empty() {
            return vec![];
        }
        get_str(key)
            .unwrap_or_default()
            .split(';')
            .filter(|s| !s.is_empty())
            .map(|s| s.trim().to_string())
            .collect()
    };
    Some(DesktopEntry {
        type_: get_str("type")?,
        name: get_str("name")?,
        generic_name: get_str("genericname"),
        comment: get_str("comment"),
        exec: get_str("exec"),
        try_exec: get_str("tryexec"),
        icon: get_str("icon"),
        categories: get_list("categories"),
        mime_type: get_list("mimetype"),
        keywords: get_list("keywords"),
        actions: get_list("actions"),
        terminal: get_bool("terminal"),
        no_display: get_bool("nodisplay"),
        hidden: get_bool("hidden"),
        other_keys: ini
            .iter()
            .filter_map(|(k, v)| match k.as_str() {
                "type" | "name" | "genericname" | "comment" | "exec" | "tryexec" | "icon"
                | "categories" | "mimeType" | "keywords" | "actions" | "terminal" | "nodisplay"
                | "hidden" => None,
                _ => v.as_ref().map(|value| (k.clone(), value.clone())),
            })
            .collect(),
    })
}

/// Reads a file and returns its metadata as `FileMetadata`.
///
/// The contents of the file are read up to `buffer_size_kb` kilobytes.
///
/// # Errors
///
/// Returns an `std::io::Error` if the file cannot be read.
pub fn get_last_modified_timestamp(
    path: &Path,
) -> Result<String, std::io::Error> {
    if let Some(ext) = path.extension() {
        if path.is_file() {
            if let Ok(metadata) = std::fs::metadata(path) {
                //Store last modified as a timestamp
                if let Ok(duration) = metadata.modified()?.duration_since(UNIX_EPOCH) {
                    return Ok(duration.as_secs().to_string())
                }
            }
        }
    }
    Ok(String::new())
}
