use std::sync::{Arc, Mutex};

use libpulse_binding::context::{Context, State};
use libpulse_binding::error::PAErr;
use libpulse_binding::mainloop::standard::Mainloop;
use libpulse_binding::volume::{ChannelVolumes, Volume};

pub struct Sound {}

impl Sound {
    pub fn new() -> Sound {
        Sound {}
    }

    pub async fn get_volume(&self) -> Result<u8, PAErr> {
        println!("Getting volume");

        let _ = run_output_command(VolumeCommand::Get);
        println!("Getting volume in get_volume function");

        Ok(0)
    }

    pub async fn set_volume(&self, volume: f64) -> Result<(), PAErr> {
        let _ = run_output_command(VolumeCommand::Set {
            value: volume as f64,
        });
        Ok(())
    }

    pub async fn mute(&self) -> Result<(), PAErr> {
        let _ = run_output_command(VolumeCommand::Mute);
        println!("Muting sound in mute function");
        Ok(())
    }

    pub async fn unmute(&self) -> Result<(), PAErr> {
        let _ = run_output_command(VolumeCommand::Unmute);
        println!("UnMuting sound in mute function");
        println!("Unmuting sound");
        Ok(())
    }

    pub async fn get_connected_devices(&self) -> Result<(), PAErr> {
        let _ = run_output_command(VolumeCommand::GetConnectedDevices);

        Ok(())
    }
}

/// Convert a [`Volume`] to a percentage as `f64`.
fn volume_to_percentage(volume: Volume) -> f64 {
    let range = Volume::NORMAL.0 as f64 - Volume::MUTED.0 as f64;
    (volume.0 as f64 - Volume::MUTED.0 as f64) * 100.0 / range
}

/// Convert a percentage to a [`Volume`].
fn percentage_to_volume(factor: f64) -> Volume {
    let range = Volume::NORMAL.0 as f64 - Volume::MUTED.0 as f64;
    Volume((Volume::MUTED.0 as f64 + factor * range / 100.0) as u32)
}

/// Volume information for a input or output device.
struct Volumes {
    /// Is the device muted?
    muted: bool,
    /// The volumes of all channels of the device.
    channels: ChannelVolumes,
}

/// Connect to a PulseAudio or PipeWire sound server.
fn connect(main_loop: &mut Mainloop) -> Result<Context, ()> {
    // Create the context.
    let mut context = libpulse_binding::context::Context::new(main_loop, "volume-control")
        .ok_or_else(|| eprintln!("Failed initialize PulseAudio context."))?;

    // Initiate the connection.
    context
        .connect(None, libpulse_binding::context::FlagSet::NOFLAGS, None)
        .map_err(|e| eprintln!("Failed to connect to PulseAudio server: {e}"))?;

    // Run the main loop until the connection succeeded or failed.
    run_until(main_loop, |_main_loop| {
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
    .map_err(|e| println!("Failed to connect to PulseAudio server: {e}"))?;

    // Check the end state to see if we connected successfully.
    let state = context.get_state();
    match state {
        State::Ready => (),
        State::Failed => {
            println!("Failed to connect to PulseAudio server.");
            return Err(());
        }
        State::Unconnected
        | State::Terminated
        | State::Connecting
        | State::Authorizing
        | State::SettingName => {
            return Err(());
        }
    }
    Ok(context)
}

/// Run the libpulse main loop until a condition becomes true.
fn run_until<F>(main_loop: &mut Mainloop, condition: F) -> Result<Option<i32>, PAErr>
where
    F: Fn(&mut Mainloop) -> bool,
{
    use libpulse_binding::mainloop::standard::IterateResult;
    loop {
        match main_loop.iterate(true) {
            IterateResult::Err(e) => {
                return Err(e);
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
fn run<F, T>(main_loop: &mut Mainloop, operation: F) -> Result<T, PAErr>
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
                return Err(e);
            }
            IterateResult::Quit(code) => {
                std::process::exit(code.0);
            }
            IterateResult::Success(_iterations) => (),
        }
    }
}

enum VolumeCommand {
    Up { value: f64 },
    Down { value: f64 },
    Set { value: f64 },
    Mute,
    Unmute,
    ToggleMute,
    Get,
    GetConnectedDevices,
}

/// Apply a function to all channel volumes.
fn map_volumes<F: FnMut(f64) -> f64>(volumes: &mut ChannelVolumes, mut action: F) {
    for volume in volumes.get_mut() {
        let factor = volume_to_percentage(*volume);
        let adjusted = action(factor).clamp(0.0, 125.0);
        *volume = percentage_to_volume(adjusted);
    }
}

// let mut main_loop = Mainloop::new()
// .ok_or_else(|| eprintln!("Failed to initialize PulseAudio main loop."))
// .unwrap();
// let context = connect(&mut main_loop).unwrap();

/// Apply a [`VolumeCommand`] to a [`Volumes`] struct.
fn apply_volume_command(volumes: &mut Volumes, command: &VolumeCommand) {
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
        VolumeCommand::GetConnectedDevices => {
            let mut main_loop = Mainloop::new()
                .ok_or_else(|| eprintln!("Failed to initialize PulseAudio main loop."))
                .unwrap();

            let mut context = connect(&mut main_loop).unwrap();

            get_connected_devices(&mut main_loop, &context).unwrap();
        }
    }
}

/// Run a volume command on the output device.
/// Run a volume command on the output device.
fn run_output_command(command: VolumeCommand) -> Result<(), ()> {
    let mut main_loop = Mainloop::new()
        .ok_or_else(|| eprintln!("Failed to initialize PulseAudio main loop."))
        .unwrap();

    let mut context = connect(&mut main_loop).unwrap();

    let mut volumes = get_output_volumes(&mut main_loop, &mut context)
        .map_err(|e| println!("Failed to get output volume: {e}"))?;

    if let VolumeCommand::Get = &command {
        let max = volume_to_percentage(volumes.channels.max());
        println!("{max:.0}");
        return Ok(());
    }

    apply_volume_command(&mut volumes, &command);

    set_output_volumes(&mut main_loop, &mut context, &volumes.channels)
        .map_err(|e| println!("Failed to set output volume: {e}"))?;
    set_output_muted(&mut main_loop, &mut context, volumes.muted)
        .map_err(|e| println!("Failed to set output mute state: {e}"))?;

    Ok(())
}

/// Get the volume of the output device.
fn get_output_volumes(main_loop: &mut Mainloop, context: &Context) -> Result<Volumes, PAErr> {
    run(main_loop, move |output| {
        context
            .introspect()
            .get_sink_info_by_name("@DEFAULT_SINK@", move |info| match info {
                libpulse_binding::callbacks::ListResult::Item(x) => {
                    *output.lock().unwrap() = Some(Ok(Volumes {
                        muted: x.mute,
                        channels: x.volume,
                    }));
                }
                libpulse_binding::callbacks::ListResult::End => {}
                libpulse_binding::callbacks::ListResult::Error => {
                    *output.lock().unwrap() = Some(Err(()));
                }
            });
    })?
    .map_err(|()| context.errno())
}

/// Get the volume of the output device.
fn set_output_volumes(
    main_loop: &mut Mainloop,
    context: &Context,
    volumes: &ChannelVolumes,
) -> Result<(), PAErr> {
    run(main_loop, move |output| {
        context.introspect().set_sink_volume_by_name(
            "@DEFAULT_SINK@",
            volumes,
            Some(Box::new(move |success| {
                if success {
                    *output.lock().unwrap() = Some(Ok(()));
                } else {
                    *output.lock().unwrap() = Some(Err(()));
                }
            })),
        );
    })?
    .map_err(|()| context.errno())
}

/// Set the muted state of the output device.
fn set_output_muted(main_loop: &mut Mainloop, context: &Context, muted: bool) -> Result<(), PAErr> {
    run(main_loop, move |output| {
        context.introspect().set_sink_mute_by_name(
            "@DEFAULT_SINK@",
            muted,
            Some(Box::new(move |success| {
                if success {
                    *output.lock().unwrap() = Some(Ok(()));
                } else {
                    *output.lock().unwrap() = Some(Err(()));
                }
            })),
        );
    })?
    .map_err(|()| context.errno())
}

fn get_connected_devices(main_loop: &mut Mainloop, context: &Context) -> Result<(), PAErr> {
    run(main_loop, move |_output| {
        context
            .introspect()
            .get_source_info_list(move |info| match info {
                libpulse_binding::callbacks::ListResult::Item(x) => {
                    println!("Name: {:?}", x.name);
                    println!("Description: {:?}", x.description);
                    println!("Sample Spec: {:?}", x.sample_spec);
                    println!("Channel Map: {:?}", x.channel_map);
                    println!("Owner Module: {:?}", x.owner_module);
                    println!("Volume: {:?}", x.volume);
                    println!("Mute: {:?}", x.mute);
                    println!("Monitor of Sink: {:?}", x.monitor_of_sink);
                    println!("Monitor of Sink Name: {:?}", x.monitor_of_sink_name);
                    println!("Latency: {:?}", x.latency);
                    println!("Driver: {:?}", x.driver);
                    println!("Flags: {:?}", x.flags);
                    println!("Proplist: {:?}", x.proplist);
                    println!("Configured Latency: {:?}", x.configured_latency);
                    println!("Base Volume: {:?}", x.base_volume);
                    println!("State: {:?}", x.state);
                    println!("Number of Volume Steps: {:?}", x.n_volume_steps);
                    println!("Card: {:?}", x.card);
                    println!("Ports: {:?}", x.ports);
                    println!("Active Port: {:?}", x.active_port);
                    println!("Formats: {:?}", x.formats);

                    println!("")
                }
                libpulse_binding::callbacks::ListResult::End => {}
                libpulse_binding::callbacks::ListResult::Error => {
                    eprintln!("Failed to get device list");
                }
            });
    })?;

    Ok(())
}

