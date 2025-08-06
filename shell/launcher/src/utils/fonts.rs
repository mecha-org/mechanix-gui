use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

#[derive(AssetCollection, Resource, Clone)]
#[allow(dead_code)]
pub struct FontAssets {
    // Fonts: Inter (primary)
    #[asset(key = "fonts.primary.100")]
    pub primary_100: Handle<Font>, // fonts/Inter_24pt-Light.ttf
    #[asset(key = "fonts.primary.200")]
    pub primary_200: Handle<Font>, // fonts/Inter_24pt-ExtraLight.ttf
    #[asset(key = "fonts.primary.300")]
    pub primary_300: Handle<Font>, // fonts/Inter_24pt-Light.ttf
    #[asset(key = "fonts.primary.400")]
    pub primary_400: Handle<Font>, // fonts/Inter_24pt-Regular.ttf
    #[asset(key = "fonts.primary.500")]
    pub primary_500: Handle<Font>, // fonts/Inter_24pt-Medium.ttf
    #[asset(key = "fonts.primary.600")]
    pub primary_600: Handle<Font>, // fonts/Inter_24pt-SemiBold.ttf
    #[asset(key = "fonts.primary.700")]
    pub primary_700: Handle<Font>, // fonts/Inter_24pt-Bold.ttf
    #[asset(key = "fonts.primary.800")]
    pub primary_800: Handle<Font>, // fonts/Inter_24pt-ExtraBold.ttf
    #[asset(key = "fonts.primary.900")]
    pub primary_900: Handle<Font>, // fonts/Inter_24pt-Black.ttf

    // Fonts: SpaceMono (secondary)
    #[asset(key = "fonts.secondary.100")]
    pub secondary_100: Handle<Font>, // fonts/SpaceMono-Regular.ttf
    #[asset(key = "fonts.secondary.200")]
    pub secondary_200: Handle<Font>, // fonts/SpaceMono-Regular.ttf
    #[asset(key = "fonts.secondary.300")]
    pub secondary_300: Handle<Font>, // fonts/SpaceMono-Regular.ttf
    #[asset(key = "fonts.secondary.400")]
    pub secondary_400: Handle<Font>, // fonts/SpaceMono-Regular.ttf
    #[asset(key = "fonts.secondary.500")]
    pub secondary_500: Handle<Font>, // fonts/SpaceMono-Regular.ttf
    #[asset(key = "fonts.secondary.600")]
    pub secondary_600: Handle<Font>, // fonts/SpaceMono-Bold.ttf
    #[asset(key = "fonts.secondary.700")]
    pub secondary_700: Handle<Font>, // fonts/SpaceMono-Bold.ttf
    #[asset(key = "fonts.secondary.800")]
    pub secondary_800: Handle<Font>, // fonts/SpaceMono-Bold.ttf
    #[asset(key = "fonts.secondary.900")]
    pub secondary_900: Handle<Font>, // fonts/SpaceMono-Bold.ttf

    // Icon Font
    #[asset(key = "fonts.icons")]
    pub font_icons: Handle<Font>, // fonts/font-icons.ttf

    // Icons & UI Images
    #[asset(key = "image.airplane_off")]
    pub airplane_off: Handle<Image>, // icons/airplane_off.png
    #[asset(key = "image.airplane_on")]
    pub airplane_on: Handle<Image>, // icons/airplane_on.png

    #[asset(key = "image.bluetooth_connected")]
    pub bluetooth_connected: Handle<Image>, // icons/bluetooth_connected.png
    #[asset(key = "image.bluetooth_none")]
    pub bluetooth_none: Handle<Image>, // icons/bluetooth_none.png
    #[asset(key = "image.bluetooth_off")]
    pub bluetooth_off: Handle<Image>, // icons/bluetooth_off.png
    #[asset(key = "image.bluetooth_on")]
    pub bluetooth_on: Handle<Image>, // icons/bluetooth_on.png
    #[asset(key = "image.bluetooth_warning")]
    pub bluetooth_warning: Handle<Image>, // icons/bluetooth_warning.png

    #[asset(key = "image.brightness_low")]
    pub brightness_low: Handle<Image>, // icons/brightness.png

    #[asset(key = "image.calculator")]
    pub calculator: Handle<Image>, // icons/calculator.png
    #[asset(key = "image.calculator_pressed")]
    pub calculator_pressed: Handle<Image>, // icons/calculator_pressed.png

    #[asset(key = "image.camera")]
    pub camera: Handle<Image>, // icons/camera.png
    #[asset(key = "image.camera_pressed")]
    pub camera_pressed: Handle<Image>, // icons/camera_pressed.png

    #[asset(key = "image.cell_signal_high")]
    pub cell_signal_high: Handle<Image>, // icons/cell_signal_high.png

    #[asset(key = "image.microphone_off")]
    pub microphone_off: Handle<Image>, // icons/microphone_off.png
    #[asset(key = "image.microphone_on")]
    pub microphone_on: Handle<Image>, // icons/microphone_on.png

    #[asset(key = "image.power_saving_off")]
    pub power_saving_off: Handle<Image>, // icons/power_saving_off.png
    #[asset(key = "image.power_saving_on")]
    pub power_saving_on: Handle<Image>, // icons/power_saving_on.png

    #[asset(key = "image.rotation_off")]
    pub rotation_off: Handle<Image>, // icons/rotation_off.png
    #[asset(key = "image.rotation_on")]
    pub rotation_on: Handle<Image>, // icons/rotation_on.png

    #[asset(key = "image.screen_recording_off")]
    pub screen_recording_off: Handle<Image>, // icons/screen_recording_off.png
    #[asset(key = "image.screen_recording_on")]
    pub screen_recording_on: Handle<Image>, // icons/screen_recording_on.png

    #[asset(key = "image.sound_low")]
    pub sound_low: Handle<Image>, // icons/sound_off.png

    #[asset(key = "image.terminal")]
    pub terminal: Handle<Image>, // icons/terminal.png

    #[asset(key = "image.wireless_high")]
    pub wireless_high: Handle<Image>, // icons/wireless_high.png
    #[asset(key = "image.wireless_low")]
    pub wireless_low: Handle<Image>, // icons/wireless_low.png
    #[asset(key = "image.wireless_medium")]
    pub wireless_medium: Handle<Image>, // icons/wireless_medium.png
    #[asset(key = "image.wireless_none")]
    pub wireless_none: Handle<Image>, // icons/wireless_none.png
    #[asset(key = "image.wireless_off")]
    pub wireless_off: Handle<Image>, // icons/wireless_off.png
    #[asset(key = "image.wireless_warning")]
    pub wireless_warning: Handle<Image>, // icons/wireless_warning.png
    #[asset(key = "image.extend_screen_none")]
    pub extend_screen_none: Handle<Image>, // icons/wireless_on.png

    // Texture Atlas Layouts
    #[asset(key = "layout.airplane")]
    pub layout_airplane: Handle<TextureAtlasLayout>,
    #[asset(key = "layout.bluetooth")]
    pub layout_bluetooth: Handle<TextureAtlasLayout>,
    #[asset(key = "layout.brightness")]
    pub layout_brightness: Handle<TextureAtlasLayout>,
    #[asset(key = "layout.calculator")]
    pub layout_calculator: Handle<TextureAtlasLayout>,
    #[asset(key = "layout.camera")]
    pub layout_camera: Handle<TextureAtlasLayout>,
    #[asset(key = "layout.cell_signal")]
    pub layout_cell_signal: Handle<TextureAtlasLayout>,
    #[asset(key = "layout.microphone")]
    pub layout_microphone: Handle<TextureAtlasLayout>,
    #[asset(key = "layout.power_saving")]
    pub layout_power_saving: Handle<TextureAtlasLayout>,
    #[asset(key = "layout.rotation")]
    pub layout_rotation: Handle<TextureAtlasLayout>,
    #[asset(key = "layout.screen_recording")]
    pub layout_screen_recording: Handle<TextureAtlasLayout>,
    #[asset(key = "layout.sound")]
    pub layout_sound: Handle<TextureAtlasLayout>,
    #[asset(key = "layout.terminal")]
    pub layout_terminal: Handle<TextureAtlasLayout>,
    #[asset(key = "layout.wireless")]
    pub layout_wireless: Handle<TextureAtlasLayout>,
    #[asset(key = "layout.extend_screen")]
    pub layout_extend_screen: Handle<TextureAtlasLayout>,
}
