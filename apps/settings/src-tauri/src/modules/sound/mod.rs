pub mod client;
use crate::error::Error;

use self::client::{SinkInformationResponse, SourceInformationResponse};

#[tauri::command]
pub async fn get_output_sound_value(device: String) -> Result<u8, Error> {
    println!( "SoundService::get_output_sound_value() {:?} ", &device);
    let sound = match client::Sound::get_output_sound_percentage(device).await {
        Ok(v) => v,
        Err(e) => return Err(Error::Other(e.to_string()))
    };

    Ok(sound as u8)
}

#[tauri::command]
pub async fn set_output_sound_value(value: u8, device: String) -> Result<(), Error> {
    println!("SoundService::set_output_sound_value() {:?} converted value {:?}",
        value,
        (value as f32) as u8
    );
    match client::Sound::set_output_sound_percentage(value as f64, device).await {
        Ok(v) => v,
        Err(e) => return Err(Error::Other(e.to_string()))
    };

    Ok(())
}


#[tauri::command]
pub async fn get_input_sound_value(device: String) -> Result<u8, Error> {
    println!( "SoundService::get_input_sound_value() {:?} ", &device);
    let sound = match client::Sound::get_input_sound_percentage(device).await {
        Ok(v) => v,
        Err(e) => return Err(Error::Other(e.to_string()))
    };

    Ok(sound as u8)
}

#[tauri::command]
pub async fn set_input_sound_value(value: u8, device: String) -> Result<(), Error> {
    println!("SoundService::set_input_sound_value() {:?} converted value {:?}",
        value,
        (value as f32) as u8
    );
    match client::Sound::set_input_sound_percentage(value as f64, device).await {
        Ok(v) => v,
        Err(e) => return Err(Error::Other(e.to_string()))
    };

    Ok(())
}


#[tauri::command]
pub async fn get_input_devices() -> Result<Vec<SinkInformationResponse>, Error> {
    println!( "SoundService::get_input_devices()");
    match client::Sound::get_input_devices().await {
        Ok(v) => return Ok(v),
        Err(e) => return Err(Error::Other(e.to_string()))
    };
}

#[tauri::command]
pub async fn get_output_devices() -> Result<Vec<SourceInformationResponse>, Error> {
    println!( "SoundService::get_output_devices()");
    match client::Sound::get_output_devices().await {
        Ok(v) => return Ok(v),
        Err(e) => return Err(Error::Other(e.to_string()))
    };
}

#[tauri::command]
pub async fn input_device_toggle_mute(device: String) -> Result<(), Error> {
    println!( "SoundService::input_device_toggle_mute()");
    match client::Sound::input_device_toggle_mute(device).await {
        Ok(v) => return Ok(v),
        Err(e) => return Err(Error::Other(e.to_string()))
    };
}

#[tauri::command]
pub async fn output_device_toggle_mute(device: String) -> Result<(), Error> {
    println!( "SoundService::output_device_toggle_mute()");
    match client::Sound::output_device_toggle_mute(device).await {
        Ok(v) => return Ok(v),
        Err(e) => return Err(Error::Other(e.to_string()))
    };
}
