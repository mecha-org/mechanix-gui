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
    #[asset(key = "image.airplane_tilt")]
    pub airplane_tilt: Handle<Image>, // icons/airplane_off.png
    #[asset(key = "image.rotation_on")]
    pub rotation_on: Handle<Image>, // icons/airplane_off.png

    #[asset(key = "image.rotation_off")]
    pub rotation_off: Handle<Image>,

    #[asset(key = "image.second_screen_on")]
    pub second_screen_on: Handle<Image>,

    #[asset(key = "image.second_screen_off")]
    pub second_screen_off: Handle<Image>,
    #[asset(key = "image.power_mode_none")]
    pub power_mode_none: Handle<Image>,
    #[asset(key = "image.power_mode_low")]
    pub power_mode_low: Handle<Image>,
    #[asset(key = "image.power_mode_high")]
    pub power_mode_high: Handle<Image>,
    #[asset(key = "image.mic_on")]
    pub mic_on: Handle<Image>,
    #[asset(key = "image.mic_off")]
    pub mic_off: Handle<Image>,
    #[asset(key = "image.screen_recording_on")]
    pub screen_recording_on: Handle<Image>,
    #[asset(key = "image.screen_recording_off")]
    pub screen_recording_off: Handle<Image>,
    #[asset(key = "image.calculator")]
    pub calculator: Handle<Image>,
    #[asset(key = "image.camera")]
    pub camera: Handle<Image>,
    #[asset(key = "image.sound_low")]
    pub sound_low: Handle<Image>,
    #[asset(key = "image.brightness_low")]
    pub brightness_low: Handle<Image>,
    #[asset(key = "image.wireless_off")]
    pub wireless_off: Handle<Image>,
    #[asset(key = "image.wireless_on")]
    pub wireless_on: Handle<Image>,
    #[asset(key = "image.bluetooth_off")]
    pub bluetooth_off: Handle<Image>,
    #[asset(key = "image.bluetooth_on")]
    pub bluetooth_on: Handle<Image>,
    #[asset(key = "image.terminal")]
    pub terminal: Handle<Image>,
    #[asset(key = "image.cell_signal_none")]
    pub cell_signal_none: Handle<Image>,
    #[asset(key = "image.battery_10")]
    pub battery_10: Handle<Image>,

}
