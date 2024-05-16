use anyhow::{anyhow, Result};
use std::sync::{Arc, Mutex};

use libpulse_binding::context::{Context, State};
use libpulse_binding::error::PAErr;
use libpulse_binding::mainloop::standard::Mainloop;
use libpulse_binding::volume::{ChannelVolumes, Volume};

pub use libpulse_binding::proplist::Proplist;

use crate::input_device::{
    get_connected_input_devices, get_input_volumes, run_input_command, SinkInformation,
};
use crate::output_device::{
    get_connected_devices, get_output_volumes, run_output_command, SourceInformation,
};

pub struct Sound {}

impl Sound {
    pub fn new() -> Sound {
        Sound {}
    }

    pub async fn get_output_device_volume(&self, device: Option<String>) -> Result<f64> {
        // Initialize the Mainloop and Context
        let (mut main_loop, mut context) = init_pulseaudio()?;

        // Get the output volumes
        let volume = match get_output_volumes(&mut main_loop, &mut context, device.clone()) {
            Ok(volume) => volume_to_percentage(volume.channels.max()),
            Err(_) => return Err(anyhow!("Failed to get volume").into()),
        };

        Ok(volume)
    }

    pub async fn set_output_device_volume(
        &self,
        volume: f64,
        device: Option<String>,
    ) -> Result<()> {
        let _ = run_output_command(
            VolumeCommand::Set {
                value: volume as f64,
            },
            Some(device.unwrap()),
        );
        Ok(())
    }

    pub async fn output_device_mute(&self, device: Option<String>) -> Result<()> {
        let _ = run_output_command(VolumeCommand::Mute, Some(device.unwrap()));
        println!("Muting sound in mute function");
        Ok(())
    }

    pub async fn output_device_unmute(&self, device: Option<String>) -> Result<()> {
        let _ = run_output_command(VolumeCommand::Unmute, Some(device.unwrap()));
        println!("UnMuting sound in mute function");
        println!("Unmuting sound");
        Ok(())
    }

    pub async fn output_device_toggle_mute(&self, device: Option<String>) -> Result<()> {
        let _ = run_output_command(VolumeCommand::ToggleMute, Some(device.unwrap()));
        println!("Toggling mute in toggle_mute function");
        Ok(())
    }

    pub async fn get_connected_output_device_list(&self) -> Result<Vec<SourceInformation>> {
        let mut main_loop = Mainloop::new()
            .ok_or_else(|| eprintln!("Failed to initialize PulseAudio main loop."))
            .unwrap();

        let mut context = connect(&mut main_loop).unwrap();

        let device_list = get_connected_devices(&mut main_loop, &mut context);

        Ok(device_list.unwrap())
    }

    pub async fn get_input_device_volume(&self, device: Option<String>) -> Result<f64> {
        // Initialize the Mainloop and Context
        let (mut main_loop, mut context) = init_pulseaudio()?;

        // Get the output volumes
        let volume = match get_input_volumes(&mut main_loop, &mut context, device.clone()) {
            Ok(volume) => volume_to_percentage(volume.channels.max()),
            Err(_) => return Err(anyhow!("Failed to get volume").into()),
        };

        Ok(volume)
    }

    pub async fn set_input_device_volume(&self, volume: f64, device: Option<String>) -> Result<()> {
        let _ = run_input_command(
            VolumeCommand::Set {
                value: volume as f64,
            },
            Some(device.unwrap()),
        );
        Ok(())
    }

    pub async fn input_device_mute(&self, device: Option<String>) -> Result<()> {
        let _ = run_input_command(VolumeCommand::Mute, Some(device.unwrap()));
        println!("Muting sound in mute function");
        Ok(())
    }

    pub async fn input_device_unmute(&self, device: Option<String>) -> Result<()> {
        let _ = run_input_command(VolumeCommand::Unmute, Some(device.unwrap()));
        println!("UnMuting sound in mute function");
        println!("Unmuting sound");
        Ok(())
    }

    pub async fn input_device_toggle_mute(&self, device: Option<String>) -> Result<()> {
        let _ = run_input_command(VolumeCommand::ToggleMute, Some(device.unwrap()));
        println!("Toggling mute in toggle_mute function");
        Ok(())
    }

    pub async fn get_connected_input_device_list(&self) -> Result<Vec<SinkInformation>> {
        let mut main_loop = Mainloop::new()
            .ok_or_else(|| eprintln!("Failed to initialize PulseAudio main loop."))
            .unwrap();

        let mut context = connect(&mut main_loop).unwrap();

        let device_list = get_connected_input_devices(&mut main_loop, &mut context);

        Ok(device_list.unwrap())
    }
}

/// Convert a [`Volume`] to a percentage as `f64`.
pub fn volume_to_percentage(volume: Volume) -> f64 {
    let range = Volume::NORMAL.0 as f64 - Volume::MUTED.0 as f64;
    (volume.0 as f64 - Volume::MUTED.0 as f64) * 100.0 / range
}

/// Convert a percentage to a [`Volume`].
pub fn percentage_to_volume(factor: f64) -> Volume {
    let range = Volume::NORMAL.0 as f64 - Volume::MUTED.0 as f64;
    Volume((Volume::MUTED.0 as f64 + factor * range / 100.0) as u32)
}

/// Volume information for a input or output device.
pub struct Volumes {
    /// Is the device muted?
    pub muted: bool,
    /// The volumes of all channels of the device.
    pub channels: ChannelVolumes,
}

/// Connect to a PulseAudio or PipeWire sound server.
pub fn connect(main_loop: &mut Mainloop) -> Result<Context> {
    // Create the context.
    let mut context = match libpulse_binding::context::Context::new(main_loop, "volume-control") {
        Some(context) => context,
        None => {
            eprintln!("Failed to create PulseAudio context.");
            return Err(anyhow!("Failed to create PulseAudio context").into());
        }
    };

    let _ = context
        .connect(None, libpulse_binding::context::FlagSet::NOFLAGS, None)
        .map_err(|e| println!("Failed to connect to PulseAudio server: {e}"));

    // Run the main loop until the connection succeeded or failed.
    let _ = run_until(main_loop, |_main_loop| {
        let state = context.get_state();
        match state {
            State::Ready => true,
            State::Failed => true,
            State::Unconnected => true,
            State::Terminated => true,
            State::Connecting => false,
            State::Authorizing => false,
            State::SettingName => false,
        }
    })
    .map_err(|e| println!("Failed to connect to PulseAudio server: {e}"));

    // Check the end state to see if we connected successfully.
    let state = context.get_state();
    match state {
        State::Ready => (),
        State::Failed => {
            println!("Failed to connect to PulseAudio server.");
            return Err(anyhow!("Failed to connect to PulseAudio server").into());
        }
        State::Unconnected
        | State::Terminated
        | State::Connecting
        | State::Authorizing
        | State::SettingName => {
            return Err(anyhow!("Failed to connect to PulseAudio server").into());
        }
    }
    Ok(context)
}

/// Run the libpulse main loop until a condition becomes true.
fn run_until<F>(main_loop: &mut Mainloop, condition: F) -> Result<Option<i32>>
where
    F: Fn(&mut Mainloop) -> bool,
{
    use libpulse_binding::mainloop::standard::IterateResult;
    loop {
        match main_loop.iterate(true) {
            IterateResult::Err(e) => {
                return Err(anyhow!("Mainloop iteration error: {:?}", e).into());
            }
            IterateResult::Quit(code) => {
                return Ok(Some(code.0));
            }
            IterateResult::Success(_iterations) => (),
        }
        if condition(main_loop) {
            return Ok(None);
        };
    }
}

/// Run the libpulse main loop until a value is set.
pub fn run<F, T>(main_loop: &mut Mainloop, operation: F) -> Result<T>
where
    F: FnOnce(Arc<Mutex<Option<T>>>),
{
    use libpulse_binding::mainloop::standard::IterateResult;
    let output = Arc::new(Mutex::new(None));
    operation(output.clone());

    loop {
        if let Some(value) = output.lock().unwrap().take() {
            return Ok(value);
        }
        match main_loop.iterate(true) {
            IterateResult::Err(e) => {
                return Err(anyhow!("Mainloop iteration error: {:?}", e));
            }
            IterateResult::Quit(code) => {
                std::process::exit(code.0);
            }
            IterateResult::Success(_iterations) => (),
        }
    }
}

pub fn init_pulseaudio() -> Result<(Mainloop, Context)> {
    let mut main_loop =
        Mainloop::new().ok_or_else(|| anyhow!("Failed to initialize PulseAudio main loop."))?;

    let context =
        connect(&mut main_loop).map_err(|_| anyhow!("Failed to connect to PulseAudio server."))?;

    Ok((main_loop, context))
}

pub enum VolumeCommand {
    Up { value: f64 },
    Down { value: f64 },
    Set { value: f64 },
    Mute,
    Unmute,
    ToggleMute,
    Get,
    GetOutputDevices,
    GetInputDevices,
}

/// Apply a function to all channel volumes.
pub fn map_volumes<F: FnMut(f64) -> f64>(volumes: &mut ChannelVolumes, mut action: F) {
    for volume in volumes.get_mut() {
        let factor = volume_to_percentage(*volume);
        let adjusted = action(factor).clamp(0.0, 125.0);
        *volume = percentage_to_volume(adjusted);
    }
}

/// Apply a [`VolumeCommand`] to a [`Volumes`] struct.
pub fn apply_volume_command(volumes: &mut Volumes, command: &VolumeCommand) {
    match command {
        VolumeCommand::Up { value } => {
            map_volumes(&mut volumes.channels, |x| x + value);
        }
        VolumeCommand::Down { value } => {
            map_volumes(&mut volumes.channels, |x| x - value);
        }
        VolumeCommand::Set { value } => {
            map_volumes(&mut volumes.channels, |_| *value);
        }
        VolumeCommand::Mute => {
            volumes.muted = true;
        }
        VolumeCommand::Unmute => {
            volumes.muted = false;
        }
        VolumeCommand::ToggleMute => {
            volumes.muted = !volumes.muted;
        }
        VolumeCommand::Get => {
            // let current_volume = volume_to_percentage(volumes.channels.max());
            // current_volume
        }
        VolumeCommand::GetOutputDevices => {
            let mut main_loop = Mainloop::new()
                .ok_or_else(|| eprintln!("Failed to initialize PulseAudio main loop."))
                .unwrap();

            let context = connect(&mut main_loop).unwrap();

            get_connected_devices(&mut main_loop, &context).unwrap();
        }

        VolumeCommand::GetInputDevices => {
            let mut main_loop = Mainloop::new()
                .ok_or_else(|| eprintln!("Failed to initialize PulseAudio main loop."))
                .unwrap();

            let context = connect(&mut main_loop).unwrap();

            get_connected_input_devices(&mut main_loop, &context).unwrap();
        }
    }
}
