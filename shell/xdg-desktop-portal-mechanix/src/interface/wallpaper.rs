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

#[derive(Clone, Copy)]
pub struct Wallpaper {}

#[derive(zvariant::SerializeDict, zvariant::Type)]
#[zvariant(signature = "a{sv}")]
pub struct WallpaperResult {
    success: bool,
}

#[derive(Clone, Debug)]
pub enum WallpaperOptions {
    SetWallpaperURI(SetWallpaperURIOptions),
    SetWallpaperFile(SetWallpaperFileOptions),
}

#[derive(zvariant::DeserializeDict, zvariant::Type, Clone, Debug)]
#[zvariant(signature = "a{sv}")]
pub struct SetWallpaperURIOptions {
    show_preview: Option<bool>,
    set_on: Option<String>,
}

#[derive(zvariant::DeserializeDict, zvariant::Type, Clone, Debug)]
#[zvariant(signature = "a{sv}")]
pub struct SetWallpaperFileOptions {
    show_preview: Option<bool>,
    set_on: Option<String>,
}

#[zbus::interface(name = "org.freedesktop.impl.portal.Wallpaper")]
impl Wallpaper {
    async fn set_wallpaper_uri(
        &self,
        handle: zvariant::ObjectPath<'_>,
        parent_window: &str,
        uri: &str,
        options: SetWallpaperURIOptions,
    ) -> PortalResponse<WallpaperResult> {
        self.run(
            handle,
            parent_window,
            WallpaperOptions::SetWallpaperURI(options),
            Some(uri.to_string()),
            None,
        )
        .await
    }

    async fn set_wallpaper_file(
        &self,
        handle: zvariant::ObjectPath<'_>,
        parent_window: &str,
        fd: i32,
        options: SetWallpaperFileOptions,
    ) -> PortalResponse<WallpaperResult> {
        self.run(
            handle,
            parent_window,
            WallpaperOptions::SetWallpaperFile(options),
            None,
            Some(fd),
        )
        .await
    }
}

impl Wallpaper {
    async fn run(
        &self,
        handle: zvariant::ObjectPath<'_>,
        parent_window: &str,
        options: WallpaperOptions,
        uri: Option<String>,
        fd: Option<i32>,
    ) -> PortalResponse<WallpaperResult> {
        dbg!(
            "wallpaper {:?}, {:?}, {:?}, {:?}, {:?}",
            &handle,
            &parent_window,
            &options,
            &uri,
            &fd
        );

        //send msg! regarding what to call
        //need to write logic for that

        // let msging: Result<(), ()> = Err(()); //this should be replaced by code that sends a msg![]

        // Box::new(|| msg!(Message::WallpaperRequested(options.clone())));

        match options {
            WallpaperOptions::SetWallpaperURI(s) => {
                // Here you would add the logic to set the wallpaper using the provided URI.
                // For now, we'll just simulate success.

                let result = WallpaperResult { success: true };

                PortalResponse::Success(result)
            }
            WallpaperOptions::SetWallpaperFile(s) => {
                // Here you would add the logic to set the wallpaper using the provided file descriptor.
                // For now, we'll just simulate success.

                let result = WallpaperResult { success: true };

                PortalResponse::Success(result)
            }
        }
    }
}
