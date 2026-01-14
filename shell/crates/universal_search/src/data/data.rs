use std::path::PathBuf;

use gpui::*;
use icons::prelude::*;

use crate::ui::models::RecentApps;

pub fn get_file_extension_icon(file_type: &str, cx: &mut App) -> PathBuf {
    let icons = Icons::global(cx).universal_search.clone();

    match file_type.to_lowercase().as_str() {
        "pdf" => icons.pdf_file,
        "zip" | "rar" | "7z" | "tar" | "gz" => icons.zip_file,
        "html" | "htm" | "xml" | "xhtml" => icons.code_file,
        "doc" | "docx" | "odt" => icons.doc_file,
        "xls" | "xlsx" | "ods" => icons.xls_file,
        "png" | "jpg" | "jpeg" | "gif" | "bmp" | "svg" | "webp" => icons.image_file,
        "mp4" | "avi" | "mov" | "mkv" | "webm" => icons.video_file,
        "mp3" | "wav" | "flac" | "aac" | "ogg" => icons.audio_file,
        "py" | "rs" | "cpp" | "cc" | "cxx" | "c" | "java" | "php" | "css" | "json" | "csv"
        | "txt" | "md" | "rtf" | "js" | "jsx" | "ts" | "tsx" => icons.code_file,
        "exe" | "app" | "deb" | "rpm" => icons.default_app,
        _ => icons.default_file,
    }
}

pub fn sample_recent_apps(cx: &mut App) -> Vec<RecentApps> {
    let icons = Icons::global(cx).universal_search.clone();

    vec![
        RecentApps {
            name: "Firefox".into(),
            icon_path: icons.firefox,
        },
        RecentApps {
            name: "Chromium".into(),
            icon_path: icons.chromium.clone(),
        },
        RecentApps {
            name: "github".into(),
            icon_path: icons.github,
        },
        RecentApps {
            name: "File Manager".into(),
            icon_path: icons.default_folder,
        },
        RecentApps {
            name: "Ardour".into(),
            icon_path: icons.ardour,
        },
        RecentApps {
            name: "Chromium".into(),
            icon_path: icons.chromium,
        },
    ]
}
