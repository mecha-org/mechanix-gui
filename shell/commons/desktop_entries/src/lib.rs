//! Enumerate installed applications.

#[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
use core::arch::aarch64::*;
use core::cmp::{self, Ordering};
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::str::FromStr;
use std::{fs, io, iter, slice};
use xdg::{BaseDirectories, BaseDirectoriesError};

#[derive(Debug)]
pub struct DesktopEntries {
    pub entries: Vec<DesktopEntry>,
}

impl DesktopEntries {
    /// Get icons for all installed applications.
    pub fn new() -> Result<Self, Error> {
        // Get all directories containing desktop files.
        let base_dirs = BaseDirectories::new()?;
        let user_dirs = base_dirs.get_data_home();
        let dirs = base_dirs.get_data_dirs();

        // Initialize icon loader.
        let loader = IconLoader::new(&dirs, "hicolor");

        let mut desktop_entries = DesktopEntries {
            entries: Vec::new(),
        };

        // Find all desktop files in these directories, then look for their icons and
        // executables.
        let mut entries = HashMap::new();
        for dir_entry in dirs
            .iter()
            .rev()
            .chain(iter::once(&user_dirs))
            .flat_map(|d| fs::read_dir(d.join("applications")).ok())
        {
            for file in dir_entry
                .filter_map(|entry| entry.ok())
                .filter(|entry| {
                    entry
                        .file_type()
                        .map_or(false, |ft| ft.is_file() || ft.is_symlink())
                })
                .filter(|entry| entry.file_name().to_string_lossy().ends_with(".desktop"))
            {
                let desktop_file = match fs::read_to_string(file.path()) {
                    Ok(desktop_file) => desktop_file,
                    Err(_) => continue,
                };

                // Ignore all groups other than the `Desktop Entry` one.
                //
                // Since `Desktop Entry` must be the first group, we just stop at the next group
                // header.
                let lines = desktop_file.lines().take_while(|line| {
                    line.trim_end() == "[Desktop Entry]" || !line.starts_with('[')
                });

                let mut icon_name = None;
                let mut exec = None;
                let mut name = None;

                // Find name, icon, and executable for the desktop entry.
                for line in lines {
                    // Get K/V pairs, allowing for whitespace around the assignment operator.
                    let (key, value) = match line.split_once('=') {
                        Some((key, value)) => (key.trim_end(), value.trim_start()),
                        None => continue,
                    };

                    match key {
                        "Name" => name = Some(value.to_owned()),
                        "Icon" => icon_name = Some(value.to_owned()),
                        "Exec" => {
                            // Remove %f/%F/%u/%U/%k variables.
                            let filtered = value
                                .split(' ')
                                .filter(|arg| !matches!(*arg, "%f" | "%F" | "%u" | "%U" | "%k"));
                            exec = Some(filtered.collect::<Vec<_>>().join(" "));
                        }
                        // Ignore explicitly hidden entries.
                        "NoDisplay" if value.trim() == "true" => {
                            exec = None;
                            break;
                        }
                        _ => (),
                    }
                }

                // Hide entries without `Exec=`.
                let exec = match exec {
                    Some(exec) => exec,
                    None => {
                        entries.remove(&file.file_name());
                        continue;
                    }
                };

                let mut exclude_entries = HashSet::new();
                exclude_entries.insert("Document Scanner".to_string());
                exclude_entries.insert("Mines".to_string());

                if exclude_entries.contains(&name.clone().unwrap_or_default()) {
                    entries.remove(&file.file_name());
                    continue;
                }

                let mut icon_path = None;
                if let Some(icon) = icon_name.clone() {
                    let path = Path::new(&icon);
                    if !path.is_absolute() {
                        if let Ok(path) = loader.icon_path(&icon, 96) {
                            icon_path = Some(path.to_path_buf())
                        };
                    } else {
                        icon_path = Some(path.to_path_buf());
                    }
                }

                if let Some(name) = name {
                    entries.insert(
                        file.file_name(),
                        DesktopEntry {
                            icon_name,
                            icon_path,
                            name,
                            exec,
                        },
                    );
                }
            }
        }
        desktop_entries.entries = entries.into_values().collect();

        // Sort entries for consistent display order.
        desktop_entries
            .entries
            .sort_unstable_by(|first, second| first.name.cmp(&second.name));

        // println!("desktop_entries {:?}", desktop_entries);

        Ok(desktop_entries)
    }

    /// Create an iterator over all applications.
    pub fn iter(&self) -> slice::Iter<'_, DesktopEntry> {
        self.entries.iter()
    }

    /// Get the desktop entry at the specified index.
    pub fn get(&self, index: usize) -> Option<&DesktopEntry> {
        self.entries.get(index)
    }

    /// Number of installed applications.
    pub fn len(&self) -> usize {
        self.entries.len()
    }
}

/// Desktop entry information.
#[derive(Debug, Clone)]
pub struct DesktopEntry {
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
                let mut dir_path = dir_entry.path();
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
                return Ok(path.as_path())
            }
            Some(first_icon) => first_icon,
            None => return Err(Error::NotFound),
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
