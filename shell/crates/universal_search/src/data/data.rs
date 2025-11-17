use crate::ui::icon::IconName;
use crate::ui::models::{FileType, RecentApps, SearchResults};

pub fn sample_files() -> Vec<SearchResults> {
    vec![
        SearchResults {
            name: "src".into(),
            file_type: FileType::Directory,
            icon_path: IconName::FolderSmall,
        },
        SearchResults {
            name: "assets".into(),
            file_type: FileType::Directory,
            icon_path: IconName::FolderSmall,
        },
        SearchResults {
            name: "components".into(),
            file_type: FileType::Directory,
            icon_path: IconName::FolderSmall,
        },
        SearchResults {
            name: "utils".into(),
            file_type: FileType::Directory,
            icon_path: IconName::FolderSmall,
        },
        SearchResults {
            name: "chromium".into(),
            file_type: FileType::App,
            icon_path: IconName::Chromium,
        },
        SearchResults {
            name: "kitty".into(),
            file_type: FileType::App,
            icon_path: IconName::File,
        },
        SearchResults {
            name: "firefox".into(),
            file_type: FileType::App,
            icon_path: IconName::Firefox,
        },
        SearchResults {
            name: "github".into(),
            file_type: FileType::App,
            icon_path: IconName::Github,
        },
        SearchResults {
            name: "main.rs".into(),
            file_type: FileType::File,
            icon_path: IconName::File,
        },
        SearchResults {
            name: "lib.json".into(),
            file_type: FileType::File,
            icon_path: IconName::File,
        },
        SearchResults {
            name: "mod.ts".into(),
            file_type: FileType::File,
            icon_path: IconName::File,
        },
        SearchResults {
            name: "test.md".into(),
            file_type: FileType::File,
            icon_path: IconName::File,
        },
    ]
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
