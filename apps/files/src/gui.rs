use fs_extra::dir::CopyOptions;
use mctk_core::component::{self, Component, RootComponent};
use mctk_core::layout::{Alignment, Dimension, Direction, Size};
use mctk_core::node;
use mctk_core::style::FontWeight;
use mctk_core::style::Styled;
use mctk_core::widgets::{Div, IconButton, IconType, Image, Text};
use mctk_core::widgets::{HDivider, Scrollable};
use mctk_core::{event, Node};
use mctk_core::{lay, msg, rect, size, size_pct, txt, Color};
use mctk_macros::{component, state_component_impl};
use std::fs;
use std::hash::Hash;
use std::io;
use std::path::PathBuf;
use std::sync::Arc;

use crate::folder_options;
use crate::modals::entry_row::EntryRow;
use crate::modals::{confirmation_modal, delete_modal, file_options, file_viewer};

pub struct ClicableIconComponent {
    pub on_click: Option<Box<dyn Fn() -> Box<Message> + Send + Sync>>,
}

impl std::fmt::Debug for ClicableIconComponent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ClicableIconComponent").finish()
    }
}

impl Component for ClicableIconComponent {
    fn on_click(&mut self, event: &mut event::Event<event::Click>) {
        if let Some(f) = &self.on_click {
            event.emit(f());
        }
    }

    fn container(&self) -> Option<Vec<usize>> {
        Some(vec![0])
    }

    fn view(&self) -> Option<Node> {
        let base = node!(
            Div::new(),
            lay![
                size_pct: [80, Auto],
                axis_alignment: Alignment::Start,
            ]
        );
        Some(base)
    }
}

#[derive(Clone)]
pub struct FileManagerParams {}

#[derive(Debug, Clone)]
pub enum Message {
    GoBack,
    SelectEntry(PathBuf),
    DeleteSelected,
    CreateFolder,
    RenameSelected,
    CopySelected,
    Paste,
    OpenModal(bool, String),
    OpenFolderOptionsModal(bool),
    OpenActionModal(bool),
    OpenDeleteModal(bool),
    ConfirmAction,
    ConfirmDelete,
    UpdateFolderName(String),
}

#[derive(Debug)]
pub struct FileManagerState {
    pub current_path: PathBuf,
    pub entries: Vec<PathBuf>,
    pub selected_file: Option<PathBuf>,
    pub copied_file: Option<PathBuf>,
    pub message: String,
    pub file_viewer_open: bool,
    pub view_file: Option<PathBuf>,
    pub file_content: Option<String>,
    pub file_is_image: bool,
    pub file_is_pdf: bool,
    pub file_no_preview: bool,
    pub is_file_action_modal_open: bool,
    pub is_folder_options_modal_open: bool,
    pub is_create_rename_modal_open: bool, // New field for the action modal
    pub action_modal_title: String,
    pub is_delete_modal_open: bool, // New field for the delete modal
    pub delete_item_name: String,
    pub folder_name: String,
    pub disable_click: bool,
}

#[component(State = "FileManagerState")]
#[derive(Debug, Default)]
pub struct FileManager {}

pub fn read_entries(path: PathBuf) -> Vec<PathBuf> {
    let mut entries = Vec::new();

    if let Ok(dir) = fs::read_dir(&path) {
        for entry in dir.flatten() {
            entries.push(entry.path());
        }
    } else {
        eprintln!("Failed to read directory: {:?}", path);
    }

    entries.sort_by(|a, b| {
        let a_is_dir = a.is_dir();
        let b_is_dir = b.is_dir();

        if a_is_dir && !b_is_dir {
            std::cmp::Ordering::Less
        } else if !a_is_dir && b_is_dir {
            std::cmp::Ordering::Greater
        } else {
            a.file_name().cmp(&b.file_name())
        }
    });

    entries
}

#[state_component_impl(FileManagerState)]
impl Component for FileManager {
    fn init(&mut self) {
        let current_path = PathBuf::from("/home/mecha");
        let entries = read_entries(current_path.clone());

        self.state = Some(FileManagerState {
            current_path,
            entries,
            selected_file: None,
            copied_file: None,
            message: String::new(),
            file_viewer_open: false,
            view_file: None,
            file_content: None,
            file_is_image: false,
            file_is_pdf: false,
            file_no_preview: false,
            is_file_action_modal_open: false,
            is_folder_options_modal_open: false,
            is_create_rename_modal_open: false, // Initialize action modal visibility
            action_modal_title: "".to_string(),
            is_delete_modal_open: false, // Initialize delete modal visibility
            delete_item_name: "".to_string(),
            folder_name: "".to_string(),
            disable_click: false,
        });
    }

    fn update(&mut self, msg: component::Message) -> Vec<component::Message> {
        if let Some(m) = msg.downcast_ref::<Message>() {
            match m {
                Message::GoBack => {
                    if self.state_ref().file_viewer_open {
                        self.state_mut().file_viewer_open = false;
                        self.state_mut().view_file = None;
                        self.state_mut().file_content = None;
                        self.state_mut().file_is_image = false;
                        self.state_mut().file_is_pdf = false;
                        self.state_mut().file_no_preview = false;
                    } else {
                        if let Some(parent) = self.state_ref().current_path.parent() {
                            self.state_mut().current_path = parent.to_path_buf();
                            self.state_mut().message = "Went back.".to_string();
                            self.state_mut().entries =
                                read_entries(self.state_ref().current_path.clone());
                        } else {
                            self.state_mut().message = "No parent directory.".to_string();
                        }
                    }
                }

                Message::SelectEntry(path) => {
                    if path.is_dir() {
                        self.state_mut().selected_file = Some(path.clone());
                        self.state_mut().current_path = path.clone();
                        self.state_mut().message = "Entered directory.".to_string();
                        self.state_mut().entries =
                            read_entries(self.state_ref().current_path.clone());
                    } else {
                        self.state_mut().selected_file = Some(path.clone());
                        self.state_mut().file_viewer_open = true;
                        self.state_mut().view_file = Some(path.clone());
                        let ext = path
                            .extension()
                            .and_then(|e| e.to_str())
                            .unwrap_or("")
                            .to_lowercase();

                        self.state_mut().file_is_image =
                            matches!(ext.as_str(), "png" | "jpg" | "jpeg" | "gif");
                        self.state_mut().file_is_pdf = ext == "pdf";
                        self.state_mut().file_no_preview = false;
                        self.state_mut().file_content = None;

                        if self.state_mut().file_is_image {
                            // Handle image loading if necessary
                        } else if self.state_mut().file_is_pdf {
                            // Handle PDF loading if necessary
                        } else if ext == "txt" {
                            match fs::read_to_string(&path) {
                                Ok(content) => {
                                    self.state_mut().file_content = Some(content);
                                }
                                Err(_) => {
                                    self.state_mut().file_no_preview = true;
                                }
                            }
                        } else {
                            if let Ok(content) = fs::read_to_string(&path) {
                                self.state_mut().file_content = Some(content);
                            } else {
                                self.state_mut().file_no_preview = true;
                            }
                        }
                    }
                }

                Message::DeleteSelected => {
                    if let Some(selected) = &self.state_ref().selected_file {
                        self.state_mut().delete_item_name = selected
                            .file_name()
                            .unwrap_or_default()
                            .to_string_lossy()
                            .to_string();
                        self.state_mut().is_delete_modal_open = true; // Open delete modal
                        self.state_mut().is_create_rename_modal_open = false;
                        self.state_mut().is_folder_options_modal_open = false;
                        self.state_mut().is_file_action_modal_open = false;
                    } else {
                        self.state_mut().message = "No file/folder selected.".to_string();
                    }
                }

                Message::CreateFolder => {
                    self.state_mut().action_modal_title = "Create Folder".to_string();
                    self.state_mut().is_create_rename_modal_open = true;
                    self.state_mut().is_file_action_modal_open = false;
                    self.state_mut().is_folder_options_modal_open = false;
                }

                Message::RenameSelected => {
                    if let Some(_selected) = &self.state_ref().selected_file {
                        self.state_mut().action_modal_title = "Rename".to_string();
                        self.state_mut().is_create_rename_modal_open = true;
                        self.state_mut().is_folder_options_modal_open = false;
                        self.state_mut().is_file_action_modal_open = false;
                    }
                }

                Message::UpdateFolderName(name) => {
                    self.state_mut().folder_name = name.clone(); // Update folder name from TextBox
                }

                Message::CopySelected => {
                    if let Some(selected) = &self.state_ref().selected_file {
                        self.state_mut().copied_file = Some(selected.clone());
                        self.state_mut().message = "Copied to clipboard.".to_string();
                    } else {
                        self.state_mut().message = "No file/folder selected.".to_string();
                    }
                }

                Message::Paste => {
                    let state = self.state_mut();
                    if let Some(copied) = &state.copied_file {
                        let dest = state.current_path.join(copied.file_name().unwrap());

                        let res: io::Result<()> = if copied.is_dir() {
                            let opts = CopyOptions::new();
                            fs_extra::dir::copy(copied, &dest, &opts)
                                .map(|_| ())
                                .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))
                        } else {
                            fs::copy(copied, &dest)
                                .map(|_| ())
                                .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))
                        };

                        match res {
                            Ok(_) => {
                                state.message = "Pasted successfully.".to_string();
                            }
                            Err(e) => {
                                state.message = format!("Error pasting: {}", e);
                            }
                        }
                    } else {
                        state.message = "No file/folder copied.".to_string();
                    }
                    self.state_mut().entries = read_entries(self.state_ref().current_path.clone());
                }

                Message::OpenModal(value, file_name) => {
                    self.state_mut().is_file_action_modal_open = *value;
                    if *value {
                        self.state_mut().disable_click = true; // Disable clicks when modal is open
                        self.state_mut().selected_file =
                            Some(self.state_ref().current_path.join(file_name));
                    } else {
                        self.state_mut().disable_click = false; // Enable clicks when modal is closed
                        self.state_mut().selected_file = None;
                    }
                }

                Message::OpenFolderOptionsModal(value) => {
                    self.state_mut().is_folder_options_modal_open = *value;
                    self.state_mut().is_file_action_modal_open = false;
                    self.state_mut().is_create_rename_modal_open = false;
                }

                Message::OpenActionModal(value) => {
                    self.state_mut().is_create_rename_modal_open = *value; // Open or close the action modal
                    self.state_mut().is_file_action_modal_open = false;
                }

                Message::OpenDeleteModal(value) => {
                    println!("OpenDeleteModal: {}", value);
                    self.state_mut().is_delete_modal_open = *value; // Open or close the action modal
                    self.state_mut().is_file_action_modal_open = false;
                }

                Message::ConfirmAction => {
                    match self.state_ref().action_modal_title.as_str() {
                        "Create Folder" => {
                            let folder_name = self.state_ref().folder_name.trim().to_string(); // Get the folder name and trim whitespace
                            if folder_name.is_empty() {
                                self.state_mut().message =
                                    "Folder name cannot be empty.".to_string();
                            } else {
                                let new_folder_path =
                                    self.state_ref().current_path.join(&folder_name);
                                if let Err(e) = fs::create_dir(&new_folder_path) {
                                    self.state_mut().message =
                                        format!("Error creating folder: {}", e);
                                } else {
                                    self.state_mut().message =
                                        format!("Created folder: {:?}", new_folder_path);
                                    self.state_mut().entries =
                                        read_entries(self.state_ref().current_path.clone());
                                }
                            }
                        }
                        "Rename" => {
                            let new_name = self.state_ref().folder_name.trim().to_string(); // Get the new name and trim whitespace
                            if new_name.is_empty() {
                                self.state_mut().message = "New name cannot be empty.".to_string();
                            } else if let Some(selected) = self.state_ref().selected_file.clone() {
                                let new_path = selected.with_file_name(new_name.clone());
                                if new_path.exists() {
                                    self.state_mut().message = format!(
                                        "A file or folder named '{}' already exists.",
                                        new_name
                                    );
                                } else {
                                    if let Err(e) = fs::rename(&selected, &new_path) {
                                        self.state_mut().message =
                                            format!("Error renaming file: {}", e);
                                    } else {
                                        self.state_mut().message =
                                            format!("Renamed to: {:?}", new_path);
                                        self.state_mut().selected_file = None;
                                        self.state_mut().current_path.pop();
                                        self.state_mut().current_path.push(new_path);
                                        self.state_mut().entries =
                                            read_entries(self.state_ref().current_path.clone());
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                    self.state_mut().is_create_rename_modal_open = false; // Close modal after action
                }
                // Handle deletion confirmation
                Message::ConfirmDelete => {
                    if let Some(selected) = self.state_ref().selected_file.clone() {
                        if selected.is_dir() {
                            match fs::remove_dir_all(&selected) {
                                Ok(_) => {
                                    self.state_mut().message =
                                        format!("Deleted: {:?}", self.state_ref().delete_item_name);
                                    self.state_mut().selected_file = None;
                                }
                                Err(e) => {
                                    self.state_mut().message = format!("Error deleting: {}", e);
                                }
                            }
                        } else {
                            match fs::remove_file(&selected) {
                                Ok(_) => {
                                    self.state_mut().message =
                                        format!("Deleted: {:?}", self.state_ref().delete_item_name);
                                    self.state_mut().selected_file = None;
                                }
                                Err(e) => {
                                    self.state_mut().message = format!("Error deleting: {}", e);
                                }
                            }
                        }
                    }

                    // Pop the current path so that deleted folder is removed
                    self.state_mut().current_path.pop();
                    self.state_mut().is_delete_modal_open = false; // Close delete modal
                    self.state_mut().entries = read_entries(self.state_ref().current_path.clone());
                }
            }
        }

        vec![]
    }

    fn render_hash(&self, hasher: &mut mctk_core::component::ComponentHasher) {
        self.state_ref().is_delete_modal_open.hash(hasher);
        self.state_ref().is_folder_options_modal_open.hash(hasher);
        self.state_ref().is_create_rename_modal_open.hash(hasher);
        self.state_ref().is_delete_modal_open.hash(hasher);
        self.state_ref().is_file_action_modal_open.hash(hasher);
    }
    fn view(&self) -> Option<mctk_core::Node> {
        let file_manager_state = self.state_ref();
        if file_manager_state.file_viewer_open {
            return Some(file_viewer::file_viewer_view(file_manager_state));
        }

        let current_path = file_manager_state.current_path.clone();
        let entries = file_manager_state.entries.clone();
        let is_modal_open = self.state_ref().is_create_rename_modal_open
            || self.state_ref().is_delete_modal_open
            || self.state_ref().is_file_action_modal_open
            || self.state_ref().is_folder_options_modal_open;

        let mut root = node!(
            Div::new().bg(Color::BLACK),
            lay![
                size_pct: [100],
                direction: Direction::Column,
                cross_alignment: Alignment::Stretch,
                padding: [5., 20., 5., 20.],
            ]
        );

        let current_folder_name = current_path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();

        let current_folder_node = node!(Text::new(txt!(current_folder_name))
            .style("color", Color::rgb(197.0, 197.0, 197.0))
            .style("size", 28.0)
            .style("line_height", 20.)
            .style("font", "Space Grotesk")
            .style("font_weight", FontWeight::Normal));

        let header_node = node!(
            Div::new().bg(Color::TRANSPARENT),
            lay![
                size: [440, 60],
                direction: Direction::Row,
                axis_alignment: Alignment::Stretch,
                cross_alignment: Alignment::Center,
                margin:[0., 10., 0., 20.],
            ]
        )
        .push(
            node!(
                Div::new().bg(Color::TRANSPARENT),
                lay![
                    size_pct: [70, Auto],
                    axis_alignment: Alignment::Start,
                    cross_alignment: Alignment::Center,
                ],
            )
            .push(node!(
                Image::new("unfold_dir_icon"),
                lay![
                    size:[24,24],
                    margin:[20.,0.,0.,10.]
                ]
            ))
            .push(
                node!(
                    Div::new(),
                    lay![
                        size_pct: [80, Auto],
                        direction: Direction::Column,
                        axis_alignment: Alignment::Start,
                    ]
                )
                .push(current_folder_node),
            ),
        )
        .push(
            node!(
                Div::new(),
                lay![
                    size_pct: [30, Auto],
                    axis_alignment: Alignment::End,
                    padding: [0, 0, 0, 8.],
                ]
            )
            .push(node!(
                IconButton::new("add_icon")
                    .on_click(Box::new(|| msg!(Message::CreateFolder)))
                    .icon_type(IconType::Png)
                    .style(
                        "size",
                        Size {
                            width: Dimension::Px(30.0),
                            height: Dimension::Px(30.0),
                        }
                    )
                    .style("background_color", Color::TRANSPARENT)
                    .style("border_color", Color::TRANSPARENT)
                    .style("active_color", Color::rgba(85., 85., 85., 0.50))
                    .style("radius", 10.),
                lay![
                    size: [42, 42],
                    axis_alignment: Alignment::End,
                    cross_alignment: Alignment::Center,
                ]
            ))
            .push(node!(
                IconButton::new("dots_icon") // Add the three-dots icon
                    .on_click(Box::new(|| msg!(Message::OpenFolderOptionsModal(true)))) // Open the options modal
                    .icon_type(IconType::Png)
                    .style(
                        "size",
                        Size {
                            width: Dimension::Px(30.0),
                            height: Dimension::Px(30.0),
                        }
                    )
                    .style("background_color", Color::TRANSPARENT)
                    .style("border_color", Color::TRANSPARENT)
                    .style("active_color", Color::rgba(85., 85., 85., 0.50))
                    .style("radius", 10.),
                lay![
                    size: [42, 42],
                    axis_alignment: Alignment::End,
                    cross_alignment: Alignment::Center,
                ]
            )),
        );
        let mut entries_div = node!(
            Div::new(),
            lay![
                size: [440, Auto],
                direction: Direction::Column,
                cross_alignment: Alignment::Stretch,
            ]
        );

        let back_row = EntryRow {
            is_file: false,
            title: "..".to_string(),
            icon_1: "fold_icon".to_string(),
            icon_2: "".to_string(),
            selected_entry: None,
            is_modal_open: is_modal_open,
        };

        if file_manager_state.is_folder_options_modal_open {
            entries_div = entries_div.push(folder_options::folder_modal_view());
        }
        if file_manager_state.is_create_rename_modal_open {
            entries_div = entries_div.push(confirmation_modal::confirmation_modal_view(
                file_manager_state,
            ));
        }

        if file_manager_state.is_delete_modal_open {
            entries_div = entries_div.push(delete_modal::delete_modal_view(file_manager_state));
        }

        if file_manager_state.is_file_action_modal_open {
            entries_div = entries_div.push(file_options::file_options_view());
        }

        entries_div = entries_div.push(node!(back_row));
        entries_div = entries_div.push(node!(HDivider {
            size: 0.5,
            color: Color::MID_GREY
        }));

        for (i, entry) in entries.iter().enumerate() {
            let name = entry
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string();
            let entry_clone = Arc::new(entry.clone());
            let (main_icon, righticon) = if entry.is_dir() {
                ("fold_icon".to_string(), "".to_string()) // Replace with actual folder icon path
            } else {
                let ext = entry
                    .extension()
                    .and_then(|e| e.to_str())
                    .unwrap_or("")
                    .to_lowercase();
                let file_icon = match ext.as_str() {
                    "txt" => "file_icon".to_string(),
                    "pdf" => "pdf_icon".to_string(),
                    "png" | "jpg" | "jpeg" | "gif" => "img_icon".to_string(),
                    _ => "file_icon".to_string(),
                };
                (file_icon, "dots_icon".to_string())
            };

            let btn_row = EntryRow {
                is_file: entry.is_file(),
                title: name.to_string(),
                icon_1: main_icon,
                icon_2: righticon,
                selected_entry: Some(entry_clone),
                is_modal_open: is_modal_open,
            };

            entries_div = entries_div.push(node!(btn_row).key(i as u64));
        }

        let mut scrollable_section = node!(
            Scrollable::new(size!(440, 360)),
            lay![
                size: [440, 360],
                direction: Direction::Column,
                cross_alignment: Alignment::Stretch,
            ]
        );
        scrollable_section = scrollable_section.push(entries_div);

        root = root.push(header_node);
        root = root.push(node!(HDivider {
            size: 1.,
            color: Color::MID_GREY
        }));
        root = root.push(scrollable_section);
        // root = root.push(actions_row);
        Some(root)
    }
}

impl RootComponent<FileManagerParams> for FileManager {}
