use crate::connections::PortalResponse;
use crate::gui::xdg_portal_handler::Message;
use mctk_core::msg;
use tokio::time::{self, Duration};
use zbus::zvariant;
use zbus::{
    fdo::Error as ZbusError,
    interface,
    zvariant::{DeserializeDict, SerializeDict, Type},
    SignalContext,
};
// use zvariant::DynamicType;
type Filter = (String, Vec<(u32, String)>);
type Filters = Vec<Filter>;
type Choices = Vec<(String, String, Vec<(String, String)>, String)>;

#[derive(Clone, Copy)]
pub struct FileChooser {}

#[derive(zvariant::SerializeDict, zvariant::Type)]
#[zvariant(signature = "a{sv}")]
pub struct FileChooserResult {
    uris: Vec<String>,
    choices: Vec<(String, String)>,
    current_filter: Option<Filter>,
}
#[derive(Clone, Debug)]
pub enum FileChooserOptions {
    OpenFile(OpenFileOptions),
    SaveFile(SaveFileOptions),
    SaveFiles(SaveFilesOptions),
}

#[derive(zvariant::DeserializeDict, zvariant::Type, Clone, Debug)]
#[zvariant(signature = "a{sv}")]
pub struct OpenFileOptions {
    accept_label: Option<String>,
    #[allow(dead_code)]
    modal: Option<bool>,
    multiple: Option<bool>,
    directory: Option<bool>,
    filters: Option<Filters>,
    current_filter: Option<Filter>,
    choices: Option<Choices>,
    current_folder: Option<Vec<u8>>,
}

#[derive(zvariant::DeserializeDict, zvariant::Type, Clone, Debug)]
#[zvariant(signature = "a{sv}")]
pub struct SaveFileOptions {
    accept_label: Option<String>,
    #[allow(dead_code)]
    modal: Option<bool>,
    filters: Option<Filters>,
    current_filter: Option<Filter>,
    choices: Option<Choices>,
    current_name: Option<String>,
    current_folder: Option<Vec<u8>>,
    #[allow(dead_code)]
    current_file: Option<Vec<u8>>,
}

#[derive(zvariant::DeserializeDict, zvariant::Type, Clone, Debug)]
#[zvariant(signature = "a{sv}")]
pub struct SaveFilesOptions {
    accept_label: Option<String>,
    #[allow(dead_code)]
    modal: Option<bool>,
    filters: Option<Filters>,
    current_filter: Option<Filter>,
    choices: Option<Choices>,
    current_name: Option<String>,
    current_folder: Option<Vec<u8>>,
    #[allow(dead_code)]
    current_file: Option<Vec<u8>>,
}

#[zbus::interface(name = "org.freedesktop.impl.portal.FileChooser")]
impl FileChooser {
    async fn open_file(
        &self,
        handle: zvariant::ObjectPath<'_>,
        app_id: &str,
        parent_window: &str,
        title: &str,
        options: OpenFileOptions,
    ) -> PortalResponse<FileChooserResult> {
        self.run(
            handle,
            app_id,
            parent_window,
            title,
            FileChooserOptions::OpenFile(options),
        )
        .await
    }

    async fn save_file(
        &self,
        handle: zvariant::ObjectPath<'_>,
        app_id: &str,
        parent_window: &str,
        title: &str,
        options: SaveFileOptions,
    ) -> PortalResponse<FileChooserResult> {
        self.run(
            handle,
            app_id,
            parent_window,
            title,
            FileChooserOptions::SaveFile(options),
        )
        .await
    }

    async fn save_files(
        &self,
        handle: zvariant::ObjectPath<'_>,
        app_id: &str,
        parent_window: &str,
        title: &str,
        options: SaveFilesOptions,
    ) -> PortalResponse<FileChooserResult> {
        self.run(
            handle,
            app_id,
            parent_window,
            title,
            FileChooserOptions::SaveFiles(options),
        )
        .await
    }
}

impl FileChooser {
    async fn run(
        &self,
        handle: zvariant::ObjectPath<'_>,
        app_id: &str,
        parent_window: &str,
        title: &str,
        options: FileChooserOptions,
    ) -> PortalResponse<FileChooserResult> {
        dbg!(
            "file chooser {:?}, {:?}, {:?}, {:?}, {:?}",
            &handle,
            &app_id,
            &parent_window,
            &title,
            &options
        );

        //send msg! regarding what to call
        //need to write logic for that

        // let msging: Result<(), ()> = Err(()); //this should be replaced by code that sends a msg![]

        // Box::new(|| msg!(Message::FileChooserRequested(options.clone())));

        match options {
            FileChooserOptions::OpenFile(s) => {
                // dbg!(&s);
                let selected_files = FileChooserResult {
                    uris: vec!["file:///home/vrn21/p.json".to_string()],
                    choices: vec![],
                    current_filter: None,
                };

                PortalResponse::Success(selected_files)
            }
            FileChooserOptions::SaveFile(s) => {
                let selected_files = FileChooserResult {
                    uris: vec![("file:///home/vrn21/Downloads/".to_owned()
                        + &s.current_name.unwrap())
                        .to_string()],
                    choices: vec![],
                    current_filter: None,
                };
                PortalResponse::Success(selected_files)
            }
            FileChooserOptions::SaveFiles(s) => {
                let selected_files = FileChooserResult {
                    uris: vec![("file:///home/vrn21/Downloads/".to_owned()
                        + &s.current_name.unwrap())
                        .to_string()],
                    choices: vec![],
                    current_filter: None,
                };
                PortalResponse::Success(selected_files)
            }
        }
    }
}
