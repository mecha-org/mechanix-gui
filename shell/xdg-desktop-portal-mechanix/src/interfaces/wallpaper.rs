use crate::connections::PortalResponse;
use zbus::zvariant;
use zbus::{
    interface,
    zvariant::{DeserializeDict, SerializeDict, Type},
};

#[derive(Clone, Copy)]
pub struct Wallpaper {}

#[derive(zvariant::SerializeDict, zvariant::Type)]
#[zvariant(signature = "a{sv}")]
pub struct WallpaperResult {
    success: bool,
}

#[derive(zvariant::DeserializeDict, zvariant::Type, Clone, Debug)]
#[zvariant(signature = "a{sv}")]
pub struct SetWallpaperURIOptions {
    show_preview: Option<bool>,
    set_on: Option<String>,
}

#[derive(Clone, Debug)]
pub enum WallpaperOptions {
    SetWallpaperURI(SetWallpaperURIOptions),
    SetWallpaperFile(SetWallpaperFileOptions),
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
        println!("set_wallpaper_uri called");
        dbg!(&handle, &parent_window, &uri, &options);

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
        println!("set_wallpaper_file called");
        dbg!(&handle, &parent_window, &fd, &options);

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

        match options {
            WallpaperOptions::SetWallpaperURI(_) => {
                // Here you would add the logic to set the wallpaper using the provided URI.
                // For now, we'll just simulate success.

                let result = WallpaperResult { success: true };

                PortalResponse::Success(result)
            }
            WallpaperOptions::SetWallpaperFile(_) => {
                // Here you would add the logic to set the wallpaper using the provided file descriptor.
                // For now, we'll just simulate success.

                let result = WallpaperResult { success: true };

                PortalResponse::Success(result)
            }
        }
    }
}
//gdbus call --session --dest org.mechanix.services --object-path /org/freedesktop/portal/desktop --method org.freedesktop.impl.portal.Wallpaper.SetWallpaperFile "/test/handle" "test_app" 10  "{'show-preview': <true>, 'set-on': <'both'>}"
