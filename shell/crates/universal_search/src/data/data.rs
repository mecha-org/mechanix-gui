use crate::ui::icon::IconName;
use crate::ui::models::RecentApps;

pub fn get_file_extension_icon(file_type: &str) -> IconName {
    match file_type.to_lowercase().as_str() {
        "pdf" => IconName::PdfFile,
        "zip" | "rar" | "7z" | "tar" | "gz" => IconName::ZipFile,
        "html" | "htm" | "xml" | "xhtml" => IconName::CodeFile,
        "doc" | "docx" | "odt" => IconName::DocFile,
        "xls" | "xlsx" | "ods" => IconName::XlsFile,
        "png" | "jpg" | "jpeg" | "gif" | "bmp" | "svg" | "webp" => IconName::ImageFile,
        "mp4" | "avi" | "mov" | "mkv" | "webm" => IconName::VideoFile,
        "mp3" | "wav" | "flac" | "aac" | "ogg" => IconName::AudioFile,
        "py" | "rs" | "cpp" | "cc" | "cxx" | "c" | "java" | "php" | "css" | "json" | "csv"
        | "txt" | "md" | "rtf" | "js" | "jsx" | "ts" | "tsx" => IconName::CodeFile,
        "exe" | "app" | "deb" | "rpm" => IconName::DefaultApp,
        _ => IconName::DefaultFile,
    }
}

pub fn sample_recent_apps() -> Vec<RecentApps> {
    vec![
        RecentApps {
            name: "Firefox".into(),
            icon_path: IconName::Firefox,
        },
        RecentApps {
            name: "Chromium".into(),
            icon_path: IconName::Chromium,
        },
        RecentApps {
            name: "github".into(),
            icon_path: IconName::Github,
        },
        RecentApps {
            name: "File Manager".into(),
            icon_path: IconName::File,
        },
        RecentApps {
            name: "Ardour".into(),
            icon_path: IconName::Ardour,
        },
        RecentApps {
            name: "Chromium".into(),
            icon_path: IconName::Chromium,
        },
    ]
}
