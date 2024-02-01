mod errors;
pub mod input;
mod lock_screen;
mod sctk;
mod settings;
mod theme;
mod widgets;
use sctk::layer_app::LayerSurfaceApp;
use smithay_client_toolkit::shell::wlr_layer::Anchor;
use std::time::Duration;
use tracing_subscriber::EnvFilter;

// Layer Surface App
fn main() -> anyhow::Result<()> {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or(EnvFilter::new("info"));

    tracing_subscriber::fmt()
        .compact()
        .with_env_filter(env_filter)
        .init();

    let (mut state, mut state_loop) = LayerSurfaceApp::new()?;

    // state.run((240, 240));

    // state_loop.run(Duration::ZERO, state, |data| {
    //     state_loop.dispatch(Duration::ZERO, &mut data);Z
    // });
    loop {
        state_loop.dispatch(Duration::ZERO, &mut state)?;
        state.dispatch_loops()?;

        if false {
            break;
        }
    }

    Ok(())
}

// XDG Window
// fn main() -> anyhow::Result<()> {
//     let env_filter = EnvFilter::try_from_default_env().unwrap_or(EnvFilter::new("debug"));

//     tracing_subscriber::fmt()
//         .compact()
//         .with_env_filter(env_filter)
//         .init();

//     let (mut state, mut state_loop) = XdgWindowApp::new()?;

//     state.run((480, 480));

//     loop {
//         state_loop.dispatch(Duration::ZERO, &mut state)?;
//         state.dispatch_loops()?;

//         if false {
//             break;
//         }
//     }

//     Ok(())
// }
