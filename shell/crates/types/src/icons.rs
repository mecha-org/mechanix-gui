use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

#[derive(AssetCollection, Resource, Clone)]
#[allow(dead_code)]
pub struct IconAssets {
    //Wireless Icons
    #[asset(path = "icons/wifi_on.png")]
    pub wifi_on: Handle<Image>,    
    #[asset(path = "icons/wifi_off.png")]
    pub wifi_off: Handle<Image>,

    #[asset(path = "icons/wifi_high.png")]
    pub wifi_high: Handle<Image>,

    #[asset(path = "icons/wifi_medium.png")]
    pub wifi_medium: Handle<Image>,

    #[asset(path = "icons/wifi_low.png")]
    pub wifi_low: Handle<Image>,

    #[asset(path = "icons/wifi_fix.png")]
    pub wifi_fix: Handle<Image>,

    //Bluetooth Icons
    #[asset(path = "icons/bluetooth_on.png")]
    pub bluetooth_on: Handle<Image>,
    #[asset(path = "icons/bluetooth_off.png")]
    pub bluetooth_off: Handle<Image>,
    #[asset(path = "icons/bluetooth_connected.png")]
    pub bluetooth_connected: Handle<Image>,

    //Battery Icons
    #[asset(path = "icons/battery_10.png")]
    pub battery_10: Handle<Image>,
    #[asset(path = "icons/battery_20.png")]
    pub battery_20: Handle<Image>,
    #[asset(path = "icons/battery_30.png")]
    pub battery_30: Handle<Image>,
    #[asset(path = "icons/battery_40.png")]
    pub battery_40: Handle<Image>,
    #[asset(path = "icons/battery_50.png")]
    pub battery_50: Handle<Image>,
    #[asset(path = "icons/battery_60.png")]
    pub battery_60: Handle<Image>,
    #[asset(path = "icons/battery_70.png")]
    pub battery_70: Handle<Image>,
    #[asset(path = "icons/battery_80.png")]
    pub battery_80: Handle<Image>,
    #[asset(path = "icons/battery_90.png")]
    pub battery_90: Handle<Image>,
    #[asset(path = "icons/battery_100.png")]
    pub battery_100: Handle<Image>,
    #[asset(path = "icons/battery_empty.png")]
    pub battery_empty: Handle<Image>,
    #[asset(path = "icons/battery_0_charging.png")]
    pub battery_0_charging: Handle<Image>,
    #[asset(path = "icons/battery_10_charging.png")]
    pub battery_10_charging: Handle<Image>,
    #[asset(path = "icons/battery_20_charging.png")]
    pub battery_20_charging: Handle<Image>,
    #[asset(path = "icons/battery_30_charging.png")]
    pub battery_30_charging: Handle<Image>,
    #[asset(path = "icons/battery_40_charging.png")]
    pub battery_40_charging: Handle<Image>,
    #[asset(path = "icons/battery_50_charging.png")]
    pub battery_50_charging: Handle<Image>,
    #[asset(path = "icons/battery_60_charging.png")]
    pub battery_60_charging: Handle<Image>,
    #[asset(path = "icons/battery_70_charging.png")]
    pub battery_70_charging: Handle<Image>,
    #[asset(path = "icons/battery_80_charging.png")]
    pub battery_80_charging: Handle<Image>,
    #[asset(path = "icons/battery_90_charging.png")]
    pub battery_90_charging: Handle<Image>,
    #[asset(path = "icons/battery_100_charging.png")]
    pub battery_100_charging: Handle<Image>,


}
