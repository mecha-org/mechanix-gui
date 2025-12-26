use freedesktop_icons::lookup;
use gdk_pixbuf::Pixbuf;
use glib::Bytes;
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::Duration;
use zbus::export::serde::{Deserialize, Serialize};
use zvariant::{OwnedValue, Structure, Type};

// check for types: https://specifications.freedesktop.org/notification/1.2/protocol.html#id-1.10.3.3.4
#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct Notification {
    pub app_name: String,
    pub replaces_id: u32,
    pub app_icon: String,
    pub summary: String,
    pub body: String,
    pub actions: Vec<String>,
    pub hints: HashMap<String, OwnedValue>,
    pub expire_timeout: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct SerializableNotification {
    pub app_name: String,
    pub replaces_id: u32,
    pub app_icon: String,
    pub summary: String,
    pub body: String,
    pub actions: Vec<String>,
    pub hints: HashMap<String, String>,
    pub expire_timeout: i32,
}

impl Notification {
    pub fn to_serializable(&self) -> SerializableNotification {
        let mut serializable_hints = HashMap::new();

        // Resolve image path if present in hints or app_icon
        if let Some(image) = self.get_image() {
            if let Ok(Some(path)) = image.resolve_path() {
                serializable_hints.insert("image-path".to_string(), path.to_string_lossy().to_string());
            }
        }

        // Convert other useful hints to string if needed
        for (key, value) in &self.hints {
            if key == "image-data" || key == "image_data" || key == "icon_data" || key == "image-path" || key == "image_path" {
                continue; // Already handled or problematic
            }

            if let Ok(v) = <OwnedValue as TryInto<String>>::try_into(value.try_clone().unwrap()) {
                 serializable_hints.insert(key.clone(), v);
            } else if let Ok(v) = <OwnedValue as TryInto<u32>>::try_into(value.try_clone().unwrap()) {
                 serializable_hints.insert(key.clone(), v.to_string());
            } else if let Ok(v) = <OwnedValue as TryInto<bool>>::try_into(value.try_clone().unwrap()) {
                 serializable_hints.insert(key.clone(), v.to_string());
            } else if let Ok(v) = <OwnedValue as TryInto<i32>>::try_into(value.try_clone().unwrap()) {
                 serializable_hints.insert(key.clone(), v.to_string());
            }
        }

        SerializableNotification {
            app_name: self.app_name.clone(),
            replaces_id: self.replaces_id,
            app_icon: self.app_icon.clone(),
            summary: self.summary.clone(),
            body: self.body.clone(),
            actions: self.actions.clone(),
            hints: serializable_hints,
            expire_timeout: self.expire_timeout,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Hints(Vec<Hint>);

#[derive(Debug, Clone)]
pub enum Hint {
    SenderPID(u32),
    ActionIcons(bool),
    Category(String),
    DesktopEntry(String),
    Image(Image),
    IconData(Vec<u8>),
    Resident(bool),
    SoundFile(PathBuf),
    SoundName(String),
    SuppressSound(bool),
    Transient(bool),
    Urgency(u8),
    X(i32),
    Y(i32),
}

pub static DEFAULT_EXPIRE_TIMEOUT: i32 = 5000; // Default timeout in milliseconds

impl Notification {
    pub fn get_hints(&self) -> Hints {
        Hints::from_hashmap(&self.hints)
    }

    pub fn get_expire_timeout(&self) -> std::time::Duration {
        if self.expire_timeout == 0 || self.is_resident() {
            Duration::from_millis(0 as u64)
        } else if self.expire_timeout == -1 {
            Duration::from_millis(DEFAULT_EXPIRE_TIMEOUT as u64)
        } else {
            Duration::from_millis(self.expire_timeout as u64)
        }
    }

    // Helper method to check if notification is resident
    pub fn is_resident(&self) -> bool {
        for (key, value) in &self.hints {
            if key == "resident" {
                if let Ok(resident) =
                    <zvariant::OwnedValue as TryInto<bool>>::try_into(value.try_clone().unwrap())
                {
                    return resident;
                }
            }
        }
        false
    }

    // Helper method to check if notification is transient
    pub fn is_transient(&self) -> bool {
        for (key, value) in &self.hints {
            if key == "transient" {
                if let Ok(transient) =
                    <zvariant::OwnedValue as TryInto<bool>>::try_into(value.try_clone().unwrap())
                {
                    return transient;
                }
            }
        }
        false
    }

    pub fn get_image(&self) -> Option<Image> {
        let hints = self.get_hints();

        // this looks for image data in hints
        for hint in &hints.0 {
            if let Hint::Image(image) = hint {
                return Some(image.clone());
            }
        }

        // If no image in hints, check app_icon field
        if !self.app_icon.is_empty() {
            // Check if app_icon is a file path
            if self.app_icon.starts_with('/') || self.app_icon.starts_with("file://") {
                let path = if self.app_icon.starts_with("file://") {
                    PathBuf::from(&self.app_icon[7..])
                } else {
                    PathBuf::from(&self.app_icon)
                };
                return Some(Image::File(path));
            } else {
                // Assume it's an icon name
                return Some(Image::Name(self.app_icon.clone()));
            }
        }

        None
    }
}

#[derive(Debug, Clone)]
pub enum Image {
    Name(String),
    File(PathBuf),
    /// RGBA/RGB Pixbuf
    Data(Pixbuf),
}

impl Image {
    pub fn save_to_path(&self, path: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            Image::Name(name) => {
                // Look up icon in theme and sav
                if let Some(icon) = lookup(name).find() {
                    // Get the best matching file path (there can be multiple for different sizes/types)
                    if let source_path = icon {
                        std::fs::copy(&source_path, &path)?;
                        println!("Saved icon to {}", source_path.display());
                    } else {
                        println!("No icon file paths found for 'firefox'");
                    }
                } else {
                    println!("No icon found for 'firefox'");
                }
                Ok(())
            }
            Image::File(file_path) => {
                // Copy existing file
                std::fs::copy(file_path, &path)?;
                Ok(())
            }
            // #todo support other formats .jpeg and .webp .svg when using savev
            Image::Data(pixbuf) => {
                let extension = path
                    .extension()
                    .and_then(|ext| ext.to_str())
                    .unwrap_or("png");

                pixbuf.savev(&path, extension, &[])?;
                Ok(())
            }
        }
    }
    pub fn resolve_path(&self) -> Result<Option<PathBuf>, Box<dyn std::error::Error>> {
        match self {
            Image::Name(name) => {
                // Look up icon in theme and sav
                if let Some(icon) = lookup(name).find() {
                    // Get the best matching file path (there can be multiple for different sizes/types)
                    return Ok(Some(icon));
                }
                Ok(None)
            }
            Image::File(file_path) => {
                Ok(Some(file_path.clone()))
            }
            // #todo support other formats .jpeg and .webp .svg when using savev
            Image::Data(pixbuf) => {
                let temp_dir = std::env::temp_dir().join("mechanix-notifications");
                if !temp_dir.exists() {
                    std::fs::create_dir_all(&temp_dir)?;
                }
                let file_name = format!("notification-{}.png", uuid::Uuid::new_v4());
                let path = temp_dir.join(file_name);
                pixbuf.savev(&path, "png", &[])?;
                Ok(Some(path))
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct ImageData {
    pub width: u32,
    pub height: u32,
    pub rowstride: i32,
    pub has_alpha: bool,
    pub bits_per_sample: i32,
    pub channels: i32,
    pub data: Vec<u8>,
}

impl Hints {
    pub fn from_hashmap(hints: &HashMap<String, OwnedValue>) -> Self {
        let mut parsed_hints = Vec::new();

        for (key, value) in hints {
            match key.as_str() {
                "action-icons" => {
                    if let Ok(val) =
                        <OwnedValue as TryInto<bool>>::try_into(value.try_clone().unwrap())
                    {
                        parsed_hints.push(Hint::ActionIcons(val));
                    }
                }
                "category" => {
                    if let Ok(val) =
                        <OwnedValue as TryInto<String>>::try_into(value.try_clone().unwrap())
                    {
                        parsed_hints.push(Hint::Category(val));
                    }
                }
                "desktop-entry" => {
                    if let Ok(val) =
                        <OwnedValue as TryInto<String>>::try_into(value.try_clone().unwrap())
                    {
                        parsed_hints.push(Hint::DesktopEntry(val));
                    }
                }
                "image-path" | "image_path" => {
                    if let Ok(val) =
                        <OwnedValue as TryInto<String>>::try_into(value.try_clone().unwrap())
                    {
                        parsed_hints.push(Hint::Image(Image::File(PathBuf::from(val))));
                    }
                }
                "image-data" | "image_data" | "icon_data" => {
                    if let Ok(structure) =
                        <OwnedValue as TryInto<Structure>>::try_into(value.try_clone().unwrap())
                    {
                        if let Ok(pixbuf) = Self::parse_image_data_to_pixbuf(structure) {
                            parsed_hints.push(Hint::Image(Image::Data(pixbuf)));
                        }
                    }
                }
                "resident" => {
                    if let Ok(val) =
                        <OwnedValue as TryInto<bool>>::try_into(value.try_clone().unwrap())
                    {
                        parsed_hints.push(Hint::Resident(val));
                    }
                }
                "sound-file" => {
                    if let Ok(val) =
                        <OwnedValue as TryInto<String>>::try_into(value.try_clone().unwrap())
                    {
                        parsed_hints.push(Hint::SoundFile(PathBuf::from(val)));
                    }
                }
                "sound-name" => {
                    if let Ok(val) =
                        <OwnedValue as TryInto<String>>::try_into(value.try_clone().unwrap())
                    {
                        parsed_hints.push(Hint::SoundName(val));
                    }
                }
                "suppress-sound" => {
                    if let Ok(val) =
                        <OwnedValue as TryInto<bool>>::try_into(value.try_clone().unwrap())
                    {
                        parsed_hints.push(Hint::SuppressSound(val));
                    }
                }
                "transient" => {
                    if let Ok(val) =
                        <OwnedValue as TryInto<bool>>::try_into(value.try_clone().unwrap())
                    {
                        parsed_hints.push(Hint::Transient(val));
                    }
                }
                "urgency" => {
                    if let Ok(val) =
                        <OwnedValue as TryInto<u8>>::try_into(value.try_clone().unwrap())
                    {
                        parsed_hints.push(Hint::Urgency(val));
                    }
                }
                "sender-pid" => {
                    if let Ok(val) =
                        <OwnedValue as TryInto<u32>>::try_into(value.try_clone().unwrap())
                    {
                        parsed_hints.push(Hint::SenderPID(val));
                    }
                }
                "x" => {
                    if let Ok(val) =
                        <OwnedValue as TryInto<i32>>::try_into(value.try_clone().unwrap())
                    {
                        parsed_hints.push(Hint::X(val));
                    }
                }
                "y" => {
                    if let Ok(val) =
                        <OwnedValue as TryInto<i32>>::try_into(value.try_clone().unwrap())
                    {
                        parsed_hints.push(Hint::Y(val));
                    }
                }
                unknown_field => {
                    eprintln!("Unknown Field:{}", unknown_field);
                    // Unknown hint, ignore
                }
            }
        }

        Hints(parsed_hints)
    }

    fn parse_image_data_to_pixbuf(
        structure: Structure,
    ) -> Result<Pixbuf, Box<dyn std::error::Error>> {
        let fields = structure.into_fields();
        if fields.len() != 7 {
            return Err("Invalid image data structure: expected 7 fields".into());
        }

        // DBus structure signature: (iiibiiay)
        let width: i32 = fields[0].try_clone()?.try_into()?;
        let height: i32 = fields[1].try_clone()?.try_into()?;
        let rowstride: i32 = fields[2].try_clone()?.try_into()?;
        let has_alpha: bool = fields[3].try_clone()?.try_into()?;
        let bits_per_sample: i32 = fields[4].try_clone()?.try_into()?;
        let channels: i32 = fields[5].try_clone()?.try_into()?;
        let data: Vec<u8> = fields[6].try_clone()?.try_into()?;

        // Validation
        if width <= 0 || height <= 0 {
            return Err("Invalid image dimensions".into());
        }

        if bits_per_sample != 8 {
            return Err("Invalid bits_per_sample: must be 8".into());
        }

        if has_alpha && channels != 4 {
            return Err("Invalid channels: must be 4 when has_alpha is true".into());
        }

        if !has_alpha && channels != 3 {
            return Err("Invalid channels: must be 3 when has_alpha is false".into());
        }

        // Create Pixbuf from raw data
        let pixbuf = Pixbuf::from_bytes(
            &Bytes::from(&data),
            gdk_pixbuf::Colorspace::Rgb,
            has_alpha,
            bits_per_sample,
            width,
            height,
            rowstride,
        );

        Ok(pixbuf)
    }
}
