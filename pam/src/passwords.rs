use entries::{Entries, Entry};
use libc::{c_long, c_ulong};
use std::fs::OpenOptions;
use std::io::{BufWriter, Error, ErrorKind, Write};
use std::num::ParseIntError;
use std::path::{Path, PathBuf};

use crate::entries;

const PASSWORDS_PATH: &str = "/etc/shadow-pins";

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct PassswordEntry {
    /// Login name
    pub name: String,

    /// Encrypted password
    pub password: String,

    /// Date of last change (measured in days since 1970-01-01 00:00:00 +0000 (UTC))
    pub last_change: c_long,

    /// Min number of days between changes
    pub min: c_long,

    /// Max number of days between changes
    pub max: c_long,

    /// Number of days before password expires to warn user to change it
    pub warning: c_long,

    /// Number of days after password expires until account is disabled
    pub inactivity: c_long,

    /// Date when account expires (measured
    /// in days since 1970-01-01 00:00:00 +0000 (UTC))
    pub expires: c_long,

    /// Reserved
    pub flag: c_ulong,
}

impl PassswordEntry {
    pub fn new<S: Into<String>>(name: S, password: S) -> Self {
        Self {
            name: name.into(),
            password: password.into(),
            last_change: 0,
            min: 0,
            max: 99999,
            warning: 7,
            inactivity: -1,
            expires: -1,
            flag: 0,
        }
    }
}

impl Entry for PassswordEntry {
    fn from_line(line: &str) -> Result<PassswordEntry, ParseIntError> {
        let parts: Vec<&str> = line.split(":").map(|part| part.trim()).collect();

        Ok(PassswordEntry {
            name: parts[0].to_string(),
            password: parts[1].to_string(),
            last_change: parts[2].parse().unwrap_or(-1),
            min: parts[3].parse().unwrap_or(-1),
            max: parts[4].parse().unwrap_or(-1),
            warning: parts[5].parse().unwrap_or(-1),
            inactivity: parts[6].parse().unwrap_or(-1),
            expires: parts[7].parse().unwrap_or(-1),
            flag: parts[8].parse().unwrap_or(0),
        })
    }
    fn to_line(&self) -> String {
        format!(
            "{}:{}:{}:{}:{}:{}:{}:{}:{}",
            self.name,
            self.password,
            self.last_change,
            self.min,
            self.max,
            self.warning,
            self.inactivity,
            self.expires,
            self.flag
        )
    }
}

pub fn get_entry_by_name_from_path(path: &Path, name: &str) -> Option<PassswordEntry> {
    Entries::<PassswordEntry>::new(path).find(|x| x.name == name)
}

pub fn get_entry_by_name(name: &str) -> Option<PassswordEntry> {
    get_entry_by_name_from_path(&Path::new(PASSWORDS_PATH), name)
}

pub fn get_all_entries_from_path(path: &PathBuf) -> Vec<PassswordEntry> {
    Entries::new(path).collect()
}

pub fn get_all_entries() -> Vec<PassswordEntry> {
    get_all_entries_from_path(&PathBuf::from(PASSWORDS_PATH))
}

pub fn entry_password_exists_by_name(name: &str) -> bool {
    let entry = get_entry_by_name_from_path(&Path::new(PASSWORDS_PATH), name);

    entry.is_some() && !entry.unwrap().password.is_empty()
}

pub fn update_or_create_entry(username: &str, new_entry: PassswordEntry) -> std::io::Result<()> {
    let mut entries: Vec<PassswordEntry> = get_all_entries();
    let mut found = false;

    for entry in &mut entries {
        if entry.name == username {
            *entry = new_entry.clone();
            found = true;
            break;
        }
    }

    if !found {
        entries.push(new_entry);
    }

    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(PASSWORDS_PATH)?;
    let mut writer = BufWriter::new(file);

    for entry in entries {
        writeln!(writer, "{}", entry.to_line())?;
    }

    Ok(())
}

pub fn remove_entry(username: &str) -> std::io::Result<()> {
    let entries: Vec<PassswordEntry> = get_all_entries();
    let entries_len = entries.len();
    let filtered_entry: Vec<PassswordEntry> = entries
        .into_iter()
        .filter(|entry| entry.name != username)
        .collect();

    if entries_len == filtered_entry.len() {
        return Err(Error::new(ErrorKind::NotFound, "username not found"));
    }

    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(PASSWORDS_PATH)?;
    let mut writer = BufWriter::new(file);

    for entry in filtered_entry {
        writeln!(writer, "{}", entry.to_line())?;
    }

    Ok(())
}
