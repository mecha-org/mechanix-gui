//! Enumerate installed applications.

#[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
use core::arch::aarch64::*;
use core::cmp::{self, Ordering};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

use std::str::FromStr;
use std::{fs, io, iter, slice};
use xdg::{BaseDirectories, BaseDirectoriesError};

#[derive(Debug)]
pub struct DesktopEntries;

impl DesktopEntries {
    /// Parse a `.desktop` file and return a `DesktopEntry`, including icon handling.
    fn parse_desktop_file(
        path: &Path,
        content: &str,
        custom_loader: &IconLoader,
        default_loader: &IconLoader,
    ) -> Result<DesktopEntry, Error> {
        let lines = content
            .lines()
            .take_while(|line| line.trim_end() == "[Desktop Entry]" || !line.starts_with('['));

        let mut icon_name = None;
        let mut exec = None;
        let mut name = None;

        for line in lines {
            let (key, value) = match line.split_once('=') {
                Some((key, value)) => (key.trim_end(), value.trim_start()),
                None => continue,
            };

            match key {
                "Name" => name = Some(value.to_owned()),
                "Icon" => icon_name = Some(value.to_owned()),
                "Exec" => {
                    let filtered = value
                        .split(' ')
                        .filter(|arg| !matches!(*arg, "%f" | "%F" | "%u" | "%U" | "%k"));
                    exec = Some(filtered.collect::<Vec<_>>().join(" "));
                }
                "NoDisplay" | "Hidden" | "Terminal" => {
                    if value.trim() == "true" {
                        return Err(Error::InvalidData);
                    }
                }
                _ => (),
            }
        }

        let exec = exec.ok_or_else(|| Error::InvalidData)?;

        let name = name.ok_or_else(|| Error::InvalidData)?;

        let mut icon_path = None;
        if let Some(icon) = &icon_name {
            let path = Path::new(icon);
            if path.is_absolute() {
                icon_path = Some(path.to_path_buf());
            } else if let Ok(path) = custom_loader.icon_path(icon, 256) {
                icon_path = Some(path.to_path_buf());
            } else if let Ok(path) = default_loader.icon_path(icon, 256) {
                icon_path = Some(path.to_path_buf());
            }
        }

        Ok(DesktopEntry {
            app_id: path
                .file_stem()
                .and_then(|os_str| os_str.to_str())
                .unwrap_or_default()
                .to_string(),
            icon_name,
            icon_path,
            name,
            exec,
        })
    }

    /// Load a `.desktop` file from a specific path.
    pub fn from_path(path: &Path) -> Result<DesktopEntry, Error> {
        // Get all directories containing icons for icon loading.
        let base_dirs = BaseDirectories::new()?;
        let dirs = base_dirs.get_data_dirs();
        let custom_loader = IconLoader::new(&dirs, "Papirus-PNG");
        let default_loader = IconLoader::new(&dirs, "hicolor");

        // Read the file content and parse it.
        let content = fs::read_to_string(path).map_err(|err| Error::InvalidData)?;
        Self::parse_desktop_file(path, &content, &custom_loader, &default_loader)
    }

    /// Get all `.desktop` files from a directory.
    fn get_desktop_files_from_dirs(dirs: &[PathBuf]) -> Vec<PathBuf> {
        dirs.iter()
            .rev()
            .flat_map(|d| fs::read_dir(d.join("applications")).ok())
            .flat_map(|dir| {
                dir.filter_map(|entry| entry.ok())
                    .filter(|entry| {
                        entry
                            .file_type()
                            .map_or(false, |ft| ft.is_file() || ft.is_symlink())
                    })
                    .filter(|entry| entry.file_name().to_string_lossy().ends_with(".desktop"))
                    .map(|entry| entry.path())
            })
            .collect()
    }

    /// Get all installed applications.
    pub fn all() -> Result<Vec<DesktopEntry>, Error> {
        // Get all directories containing desktop files.
        let base_dirs = BaseDirectories::new()?;
        let user_dir = base_dirs.get_data_home();
        let mut dirs = base_dirs.get_data_dirs();
        dirs.push(user_dir);

        // Initialize icon loaders.
        let custom_loader = IconLoader::new(&dirs, "Papirus-PNG");
        let default_loader = IconLoader::new(&dirs, "hicolor");

        // Find all `.desktop` files.
        let desktop_files = Self::get_desktop_files_from_dirs(&dirs);

        // Parse each `.desktop` file.
        let mut desktop_entries = Vec::new();
        let exclude_entries = HashSet::from(["Document Scanner".to_string(), "Mines".to_string()]);

        for path in desktop_files {
            if let Ok(content) = fs::read_to_string(&path) {
                match Self::parse_desktop_file(&path, &content, &custom_loader, &default_loader) {
                    Ok(entry) => {
                        if !exclude_entries.contains(&entry.name) {
                            desktop_entries.push(entry);
                        }
                    }
                    Err(_) => continue,
                }
            }
        }

        // Sort entries for consistent display order.
        desktop_entries.sort_unstable_by(|first, second| first.name.cmp(&second.name));

        Ok(desktop_entries)
    }

    pub fn get_dirs() -> Result<Vec<PathBuf>, Error> {
        let base_dirs = BaseDirectories::new()?;
        let user_dir = base_dirs.get_data_home();
        let mut dirs = base_dirs.get_data_dirs();
        dirs.push(user_dir);
        Ok(dirs)
    }
}

/// Desktop entry information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DesktopEntry {
    pub app_id: String,
    pub icon_name: Option<String>,
    pub icon_path: Option<PathBuf>,
    pub name: String,
    pub exec: String,
}

/// Rendered icon.
#[derive(Debug, Clone)]
pub struct Icon {
    pub data: Vec<u8>,
    pub width: usize,
}

/// Expected type of an image.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum ImageType {
    /// A bitmap image of a known square size.
    SizedBitmap(u32),

    /// A bitmap image of an unknown size.
    Bitmap,

    /// A vector image.
    Scalable,

    /// A monochrome vector image.
    Symbolic,
}

impl Ord for ImageType {
    fn cmp(&self, other: &Self) -> Ordering {
        if self == other {
            return Ordering::Equal;
        }

        match (self, other) {
            // Prefer scaleable formats.
            (Self::Scalable, _) => Ordering::Greater,
            (_, Self::Scalable) => Ordering::Less,
            // Prefer bigger bitmap sizes.
            (Self::SizedBitmap(size), Self::SizedBitmap(other_size)) => size.cmp(other_size),
            // Prefer bitmaps with known size.
            (Self::SizedBitmap(_), _) => Ordering::Greater,
            (_, Self::SizedBitmap(_)) => Ordering::Less,
            // Prefer bitmaps over symbolic icons without color.
            (Self::Bitmap, _) => Ordering::Greater,
            (_, Self::Bitmap) => Ordering::Less,
            // Equality is checked by the gate clause already.
            (Self::Symbolic, Self::Symbolic) => unreachable!(),
        }
    }
}

impl PartialOrd for ImageType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Simple loader for app icons.
#[derive(Debug)]
struct IconLoader {
    icons: HashMap<String, HashMap<ImageType, PathBuf>>,
}

impl IconLoader {
    /// Initialize the icon loader.
    ///
    /// This will check all paths for available icons and store them for cheap
    /// lookup.
    fn new(data_dirs: &[PathBuf], theme_name: &str) -> Self {
        let mut icons: HashMap<String, HashMap<ImageType, PathBuf>> = HashMap::new();

        // Iterate on all XDG_DATA_DIRS to look for icons.
        for data_dir in data_dirs {
            // Get icon directory location in the default theme.
            //
            // NOTE: In the future, we might want to parse the index.theme of the theme we
            // want to load, to handle the proper inheritance hierarchy.
            let mut icons_dir = data_dir.to_owned();
            icons_dir.push("icons");
            icons_dir.push(theme_name);

            for dir_entry in fs::read_dir(icons_dir).into_iter().flatten().flatten() {
                // Get last path segment from directory.
                let dir_name = match dir_entry.file_name().into_string() {
                    Ok(dir_name) => dir_name,
                    Err(_) => continue,
                };

                // Handle standardized icon theme directory layout.
                let image_type = if dir_name == "scalable" {
                    ImageType::Scalable
                } else if dir_name == "symbolic" {
                    ImageType::Symbolic
                } else if let Some((width, height)) = dir_name.split_once('x') {
                    match (u32::from_str(width), u32::from_str(height)) {
                        (Ok(width), Ok(height)) if width == height => ImageType::SizedBitmap(width),
                        _ => continue,
                    }
                } else {
                    continue;
                };

                // Get the directory storing the icons themselves.
                let mut dir_path = dir_entry.path().clone();
                dir_path.push("apps");

                for file in fs::read_dir(dir_path).into_iter().flatten().flatten() {
                    // Get last path segment from file.
                    let file_name = match file.file_name().into_string() {
                        Ok(file_name) => file_name,
                        Err(_) => continue,
                    };

                    // Strip extension.
                    let name = match (file_name.rsplit_once('.'), image_type) {
                        (Some((name, _)), ImageType::Symbolic) => {
                            match name.strip_prefix("-symbolic") {
                                Some(name) => name,
                                None => continue,
                            }
                        }
                        (Some((name, _)), _) => name,
                        (None, _) => continue,
                    };

                    // Add icon to our icon loader.
                    icons
                        .entry(name.to_owned())
                        .or_default()
                        .insert(image_type, file.path());
                }

                // Get the directory storing the icons themselves.
                let mut dir_path = dir_entry.path().clone();
                dir_path.push("categories");

                for file in fs::read_dir(dir_path).into_iter().flatten().flatten() {
                    // Get last path segment from file.
                    let file_name = match file.file_name().into_string() {
                        Ok(file_name) => file_name,
                        Err(_) => continue,
                    };

                    // Strip extension.
                    let name = match (file_name.rsplit_once('.'), image_type) {
                        (Some((name, _)), ImageType::Symbolic) => {
                            match name.strip_prefix("-symbolic") {
                                Some(name) => name,
                                None => continue,
                            }
                        }
                        (Some((name, _)), _) => name,
                        (None, _) => continue,
                    };

                    // Add icon to our icon loader.
                    icons
                        .entry(name.to_owned())
                        .or_default()
                        .insert(image_type, file.path());
                }
            }
        }

        // This path is hardcoded in the specification.
        for file in fs::read_dir("/usr/share/pixmaps")
            .into_iter()
            .flatten()
            .flatten()
        {
            // Get last path segment from file.
            let file_name = match file.file_name().into_string() {
                Ok(file_name) => file_name,
                Err(_) => continue,
            };

            // Determine image type based on extension.
            let (name, image_type) = match file_name.rsplit_once('.') {
                Some((name, "svg")) => (name, ImageType::Scalable),
                // We don’t have any information about the size of the icon here.
                Some((name, "png")) => (name, ImageType::Bitmap),
                _ => continue,
            };

            // Add icon to our icon loader.
            icons
                .entry(name.to_owned())
                .or_default()
                .insert(image_type, file.path());
        }

        Self { icons }
    }

    /// Get the ideal icon for a specific size.
    fn icon_path<'a>(&'a self, icon: &str, size: u32) -> Result<&'a Path, Error> {
        // Get all available icons matching this icon name.
        let icons = self.icons.get(icon).ok_or(Error::NotFound)?;
        let mut icons = icons.iter();

        // Initialize accumulator with the first iterator item.
        let mut ideal_icon = match icons.next() {
            // Short-circuit if the first icon is an exact match.
            Some((ImageType::SizedBitmap(icon_size), path)) if *icon_size == size => {
                return Ok(path.as_path());
            }
            Some(first_icon) => first_icon,
            None => {
                return Err(Error::NotFound);
            }
        };

        // Find the ideal icon.
        for icon in icons {
            // Short-circuit if an exact size match exists.
            if matches!(icon, (ImageType::SizedBitmap(icon_size), _) if *icon_size == size) {
                return Ok(icon.1);
            }

            // Otherwise find closest match.
            ideal_icon = cmp::max(icon, ideal_icon);
        }

        Ok(ideal_icon.1.as_path())
    }
}

/// Icon loading error.
#[derive(Debug)]
pub enum Error {
    BaseDirectories(BaseDirectoriesError),
    Io(io::Error),
    NotFound,
    InvalidData,
}

impl From<BaseDirectoriesError> for Error {
    fn from(error: BaseDirectoriesError) -> Self {
        Self::BaseDirectories(error)
    }
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Self::Io(error)
    }
}
