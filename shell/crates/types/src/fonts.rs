use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

#[derive(AssetCollection, Resource, Clone)]
#[allow(dead_code)]
pub struct FontAssets {
    // Fonts: Inter (primary)
    #[asset(key = "fonts.primary.100")]
    pub primary_100: Handle<Font>, // fonts/Inter_24pt-Light.ttf
    #[asset(key = "fonts.primary.200")]
    pub primaiconsry_200: Handle<Font>, // fonts/Inter_24pt-ExtraLight.ttf
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
}
