use anyhow::{bail, Result};
use serde::{de, Deserialize, Deserializer, Serialize};
use std::{
    collections::HashSet,
    fs::{self, read_dir, File},
    path::{Path, PathBuf},
    process::Command,
    str::FromStr,
};

use tracing::{error, info};

use crate::errors::{DesktopIniError, DesktopIniErrorCodes};
mod errors;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DesktopEntryIni {
    #[serde(rename = "Desktop Entry")]
    pub content: DesktopEntry,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct DesktopEntry {
    pub name: Option<String>,
    pub exec: Option<String>,
    pub icon: Option<String>,
    pub actions: Option<String>,
    pub comment: Option<String>,
    pub keywords: Option<String>,
    pub no_display: Option<String>,
    pub terminal: Option<String>,
    #[serde(default)]
    #[serde(deserialize_with = "semicolon_deserialize")]
    pub only_show_in: Option<Vec<String>>,
    #[serde(default)]
    #[serde(deserialize_with = "semicolon_deserialize")]
    pub not_show_in: Option<Vec<String>>,
}

fn semicolon_deserialize<'de, D>(deserializer: D) -> Result<Option<Vec<String>>, D::Error>
where
    D: Deserializer<'de>,
{
    let str_sequence = String::deserialize(deserializer)?;
    Ok(Some(
        str_sequence
            .split(';')
            .filter(|str| str.len() > 0)
            .map(|item| item.to_owned())
            .collect(),
    ))
}

pub fn get_all_files_paths_in_directory(directory_path: &str) -> Result<Vec<String>> {
    let mut file_paths = Vec::new();
    let path = Path::new(directory_path);
    let read_dir = match read_dir(path) {
        Ok(v) => v,
        Err(e) => {
            bail!(DesktopIniError::new(
                DesktopIniErrorCodes::DirectoryReadError,
                format!("cannot read directory - {}", e.to_string()),
            ));
        }
    };

    read_dir
        .into_iter()
        .for_each(|dir_entry_res| match dir_entry_res {
            Ok(dir_entry) => {
                let entry_path = dir_entry.path();
                if entry_path.is_file() {
                    file_paths.push(String::from(entry_path.to_str().unwrap()));
                }
            }
            Err(e) => {
                error!("error while getting reading dir {}", e);
            }
        });
    Ok(file_paths)
}

pub fn get_desktop_entries(entries_path: &str) -> Vec<DesktopEntry> {
    let mut applications: Vec<DesktopEntry> = Vec::new();

    let files_in_entries_path = match get_all_files_paths_in_directory(entries_path) {
        Ok(v) => v,
        Err(e) => Vec::new(),
    };

    files_in_entries_path.into_iter().for_each(|entry_path| {
        let desktop_entry_op = match read_desktop_file(&entry_path) {
            Ok(app) => Some(app),
            Err(e) => {
                error!("error while getting desktop application {}", e);
                None
            }
        };
        match desktop_entry_op {
            Some(desktop_entry) => {
                applications.push(desktop_entry);
            }
            None => (),
        }
    });

    applications
}

pub fn read_desktop_file(file_path: &str) -> Result<DesktopEntry> {
    let file_path = PathBuf::from(String::from(file_path));

    info!(
        task = "read_desktop_file",
        "desktop file location - {:?}", file_path
    );

    // open file
    let desktop_file_handle = match fs::read_to_string(file_path) {
        Ok(file) => file,
        Err(e) => {
            bail!(DesktopIniError::new(
                DesktopIniErrorCodes::FileReadError,
                format!("cannot read the .desktop in the path - {}", e.to_string()),
            ));
        }
    };

    info!("desktop_file_handle {:?}", desktop_file_handle);

    // read and parse
    let desktop_entry_ini: DesktopEntryIni = match serde_ini::from_str(&desktop_file_handle) {
        Ok(desktop_entry_ini) => desktop_entry_ini,
        Err(e) => {
            bail!(DesktopIniError::new(
                DesktopIniErrorCodes::ParseInfoError,
                format!("error parsing the .desktop - {}", e.to_string()),
            ));
        }
    };

    info!("desktop_entry is {:?}", desktop_entry_ini.content);

    Ok(desktop_entry_ini.content)
}
