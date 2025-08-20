use bevy::prelude::*;
use bevy::render::render_asset::RenderAssetUsages;
use bevy::render::render_resource::{Extent3d, TextureDimension,TextureFormat};
use std::time::{ Duration, SystemTime, UNIX_EPOCH };
use image::{ codecs::png::PngEncoder, ImageEncoder };
use tokio::sync::{ mpsc, oneshot };
use wayland_protocols_async::zwlr_screencopy_v1::handler::{
    ScreencopyHandler,
    ScreencopyMessage,
    ScreencopyFrameOutput,
};
use std::path::PathBuf;

#[derive(Resource)]
pub struct ScreenshotManager {
    runtime: Option<tokio::runtime::Runtime>,
    sender: Option<mpsc::Sender<ScreenshotRequest>>,
    frame_receiver: Option<mpsc::Receiver<(ScreencopyFrameOutput, ScreenshotEvent)>>,
}

struct ScreenshotRequest {
    event: ScreenshotEvent,
    response: oneshot::Sender<Result<(), String>>,
}

impl Default for ScreenshotManager {
    fn default() -> Self {
        Self {
            runtime: None,
            sender: None,
            frame_receiver: None,
        }
    }
}

#[derive(Event, Clone, Debug)]
pub struct ScreenshotEvent {
    pub capture_type: CaptureType,
    pub mouse_overlay: MouseOverlay,
    pub output_path: PathBuf,
}

#[derive(Resource)]
pub struct CurrentScreenshotImage {
    pub image: Handle<Image>,
    pub output_path: PathBuf,
}

#[derive(Clone, Debug)]
pub enum CaptureType {
    FullScreen,
    Region {
        x: u32,
        y: u32,
        width: u32,
        height: u32,
    },
}

#[derive(Clone, Debug)]
pub enum MouseOverlay {
    Include,
    Exclude,
}

impl Default for ScreenshotEvent {
    fn default() -> Self {
        Self {
            capture_type: CaptureType::FullScreen,
            mouse_overlay: MouseOverlay::Include,
            output_path: get_screenshot_path(),
        }
    }
}

impl ScreenshotEvent {
    pub fn full_screen() -> Self {
        Self::default()
    }

    pub fn region(x: u32, y: u32, width: u32, height: u32) -> Self {
        Self {
            capture_type: CaptureType::Region { x, y, width, height },
            mouse_overlay: MouseOverlay::Include,
            output_path: get_screenshot_path(),
        }
    }

    pub fn with_mouse_overlay(mut self, overlay: MouseOverlay) -> Self {
        self.mouse_overlay = overlay;
        self
    }

    pub fn with_output_path(mut self, path: PathBuf) -> Self {
        self.output_path = path;
        self
    }
}

const DEFAULT_SCREENSHOT_DIR: &str = "$HOME/Pictures/Screenshots";

fn get_screenshot_path() -> PathBuf {
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

    match std::env::var("XDG_PICTURES_DIR") {
        Ok(dir) => {
            let path = PathBuf::from(dir).join("Screenshots");
            std::fs::create_dir_all(&path).ok();
            path.join(format!("screenshot-{}.png", timestamp))
        }
        Err(_) => {
            if let Some(home) = std::env::var_os("HOME") {
                let path = PathBuf::from(home).join("Pictures").join("Screenshots");
                std::fs::create_dir_all(&path).ok();
                path.join(format!("screenshot-{}.png", timestamp))
            } else {
                eprintln!(
                    "Could not determine home directory for saving screenshot (Set $XDG_PICTURES_DIR)"
                );
                PathBuf::from(DEFAULT_SCREENSHOT_DIR).join(format!("screenshot-{}.png", timestamp))
            }
        }
    }
}

/// Convert BGRA pixel data to RGBA format by swapping blue and red channels
fn bgra_to_rgba(bgra_data: &[u8]) -> Vec<u8> {
    let mut rgba_data = Vec::with_capacity(bgra_data.len());
    
    // Process 4 bytes at a time (BGRA -> RGBA)
    for chunk in bgra_data.chunks_exact(4) {
        if chunk.len() == 4 {
            let b = chunk[0];
            let g = chunk[1];
            let r = chunk[2];
            let a = chunk[3];
            
            // Swap B and R channels: BGRA -> RGBA
            rgba_data.extend_from_slice(&[r, g, b, a]);
        }
    }
    
    rgba_data
}

pub struct ScreenshotPlugin;

impl Plugin for ScreenshotPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ScreenshotManager::default())
            .add_event::<ScreenshotEvent>()
            .add_systems(Startup, setup_screenshot_handler)
            .add_systems(Update, (
                handle_screenshot_input, 
                handle_screenshot_events,
                process_screenshot_frames
            ));
    }
}

fn setup_screenshot_handler(mut screenshot_manager: ResMut<ScreenshotManager>) {
    // Create a Tokio runtime for handling Wayland operations
    match tokio::runtime::Runtime::new() {
        Ok(runtime) => {
            let (request_tx, mut request_rx) = mpsc::channel::<ScreenshotRequest>(32);
            let (frame_tx, frame_rx) = mpsc::channel::<(ScreencopyFrameOutput, ScreenshotEvent)>(32);

            // Spawn the persistent screenshot handler
            runtime.spawn(async move {
                println!("Starting persistent screenshot handler...");

                // Create channels for the screencopy handler
                let (screencopy_msg_tx, screencopy_msg_rx) = mpsc::channel(128);
                let (screencopy_event_tx, mut screencopy_event_rx) = mpsc::channel(128);

                // Create the screencopy handler
                let mut screencopy_handler = ScreencopyHandler::new(screencopy_event_tx);

                // Start the handler in the background
                let handler_task = tokio::spawn(async move {
                    screencopy_handler.run(screencopy_msg_rx).await;
                });

                // Start event processing with better handling
                let event_task = tokio::spawn(async move {
                    let mut ready_received = false;
                    while let Some(msg) = screencopy_event_rx.recv().await {
                        println!("Received screencopy event: {:?}", msg);

                        // Track when the frame is actually ready
                        match msg {
                            wayland_protocols_async::zwlr_screencopy_v1::handler::ScreencopyEvent::Ready {
                                ..
                            } => {
                                ready_received = true;
                                println!("Frame is ready for processing");
                            }
                            _ => {}
                        }
                    }
                });

                // Process screenshot requests
                while let Some(request) = request_rx.recv().await {
                    let result = handle_screenshot_request(&screencopy_msg_tx, request.event, &frame_tx).await;
                    let _ = request.response.send(result);
                }

                // Clean up tasks when done
                handler_task.abort();
                event_task.abort();
            });

            screenshot_manager.runtime = Some(runtime);
            screenshot_manager.sender = Some(request_tx);
            screenshot_manager.frame_receiver = Some(frame_rx);
            println!("Screenshot plugin initialized with persistent handler");
        }
        Err(e) => {
            eprintln!("Failed to create Tokio runtime for screenshots: {:?}", e);
        }
    }
}

async fn handle_screenshot_request(
    screencopy_msg_tx: &mpsc::Sender<ScreencopyMessage>,
    event: ScreenshotEvent,
    bevy_app_sender: &mpsc::Sender<(ScreencopyFrameOutput, ScreenshotEvent)>
) -> Result<(), String> {
    println!("Processing screenshot request: {:?}", event);

    // Determine mouse overlay setting
    let overlay_cursor = match event.mouse_overlay {
        MouseOverlay::Include => Some(1),
        MouseOverlay::Exclude => Some(0),
    };

    // Step 1: Capture output
    let (tx, rx) = oneshot::channel();
    let capture_message = ScreencopyMessage::CaptureOutput {
        overlay_cursor,
        reply_to: tx,
    };

    screencopy_msg_tx
        .send(capture_message).await
        .map_err(|e| format!("Failed to send capture message: {:?}", e))?;

    rx.await
        .map_err(|e| format!("Failed to receive capture response: {:?}", e))?
        .map_err(|e| format!("Capture failed: {:?}", e))?;

    println!("Screen capture initiated successfully");

    // Wait longer for the capture to complete
    tokio::time::sleep(Duration::from_millis(500)).await;

    // Step 2: Copy frame
    let (tx, rx) = oneshot::channel();
    screencopy_msg_tx
        .send(ScreencopyMessage::CopyFrame { reply_to: tx }).await
        .map_err(|e| format!("Failed to send copy frame message: {:?}", e))?;

    match rx.await {
        Ok(Ok(frame_output)) => {
            // Wait a bit more to ensure frame data is fully available
            tokio::time::sleep(Duration::from_millis(100)).await;

            // Validate frame data before processing
            match &frame_output.file {
                Ok(data) => {
                    if data.len() == 0 {
                        return Err("Frame data is empty".to_string());
                    }
                    println!(
                        "Frame data size: {} bytes, dimensions: {}x{}",
                        data.len(),
                        frame_output.width,
                        frame_output.height
                    );
                }
                Err(e) => {
                    return Err(format!("Frame data error: {:?}", e));
                }
            }

            // Send frame data to Bevy app for processing
            bevy_app_sender.send((frame_output, event)).await
                .map_err(|e| format!("Failed to send frame to Bevy app: {:?}", e))?;

            Ok(())
        }
        Ok(Err(e)) => Err(format!("Failed to copy frame: {:?}", e)),
        Err(e) => Err(format!("Failed to receive frame response: {:?}", e)),
    }
}

fn handle_screenshot_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut screenshot_events: EventWriter<ScreenshotEvent>
) {
    if keys.just_pressed(KeyCode::PrintScreen) {
        screenshot_events.write(ScreenshotEvent::full_screen());
    }
}

fn handle_screenshot_events(
    mut screenshot_events: EventReader<ScreenshotEvent>,
    screenshot_manager: Res<ScreenshotManager>
) {
    for event in screenshot_events.read() {
        if let Some(sender) = &screenshot_manager.sender {
            let (response_tx, response_rx) = oneshot::channel();
            let request = ScreenshotRequest {
                event: event.clone(),
                response: response_tx,
            };

            if let Some(runtime) = &screenshot_manager.runtime {
                let sender = sender.clone();
                runtime.spawn(async move {
                    if let Err(e) = sender.send(request).await {
                        eprintln!("Failed to send screenshot request: {:?}", e);
                    }

                    // Optionally wait for response
                    match response_rx.await {
                        Ok(Ok(())) => {}
                        Ok(Err(e)) => eprintln!("Screenshot failed: {}", e),
                        Err(e) => eprintln!("Failed to receive screenshot response: {:?}", e),
                    }
                });
            }
        } else {
            eprintln!("Screenshot system not initialized");
        }
    }
}

fn process_screenshot_frames(
    mut screenshot_manager: ResMut<ScreenshotManager>,
    mut images: ResMut<Assets<Image>>,
    mut commands: Commands,
) {
    if let Some(frame_receiver) = screenshot_manager.frame_receiver.as_mut() {
        while let Ok((frame_output, event)) = frame_receiver.try_recv() {
            if let Ok(data) = &frame_output.file {
                // Create Bevy Image from frame data
                let (final_data, final_width, final_height) = match &event.capture_type {
                    CaptureType::FullScreen => {
                        let data_vec = data.to_vec();
                        (data_vec, frame_output.width, frame_output.height)
                    }
                    CaptureType::Region { x, y, width, height } => {
                        let data_slice: &[u8] = &data;
                        let cropped_data = crop_image_data(
                            data_slice,
                            frame_output.width,
                            frame_output.height,
                            *x,
                            *y,
                            *width,
                            *height
                        );
                        (cropped_data, *width, *height)
                    }
                };
                let final_data = bgra_to_rgba(&final_data);
                // Create Bevy Image
                let bevy_image = Image::new(
                    Extent3d {
                        width: final_width,
                        height: final_height,
                        depth_or_array_layers: 1,
                    },
                    TextureDimension::D2,
                    final_data,
                    TextureFormat::Rgba8UnormSrgb,
                    RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
                );

                // Add image to assets and get handle
                let image_handle = images.add(bevy_image);

                // Store current screenshot data
                commands.insert_resource(CurrentScreenshotImage {
                    image: image_handle,
                    output_path: event.output_path,
                });

                println!("Screenshot captured and converted to Bevy Image");
            }
        }
    }
}

pub fn trigger_screenshot(screenshot_manager: &ScreenshotManager, event: ScreenshotEvent) {
    if let Some(sender) = &screenshot_manager.sender {
        let (response_tx, response_rx) = oneshot::channel();
        let request = ScreenshotRequest {
            event,
            response: response_tx,
        };

        if let Some(runtime) = &screenshot_manager.runtime {
            let sender = sender.clone();
            runtime.spawn(async move {
                if let Err(e) = sender.send(request).await {
                    eprintln!("Failed to send screenshot request: {:?}", e);
                    return;
                }

                match response_rx.await {
                    Ok(Ok(())) => println!("Screenshot completed successfully"),
                    Ok(Err(e)) => eprintln!("Screenshot failed: {}", e),
                    Err(e) => eprintln!("Failed to receive screenshot response: {:?}", e),
                }
            });
        }
    } else {
        eprintln!("Screenshot system not initialized");
    }
}

fn write_frame_to_file(frame: ScreencopyFrameOutput, event: &ScreenshotEvent) {
    let file_name = event.output_path.clone();

    // Ensure the parent directory exists
    if let Some(parent) = file_name.parent() {
        std::fs::create_dir_all(parent).ok();
    }

    match std::fs::File::create(&file_name) {
        Ok(mut writer) => {
            let png_encoder = PngEncoder::new(&mut writer);

            match frame.file {
                Ok(data) => {
                    // Handle region capture by cropping the data if needed
                    let (final_data, final_width, final_height) = match &event.capture_type {
                        CaptureType::FullScreen => {
                            // Convert MmapMut to Vec<u8> for consistency
                            let data_vec = data.to_vec();
                            (data_vec, frame.width, frame.height)
                        }
                        CaptureType::Region { x, y, width, height } => {
                            // Convert MmapMut to slice, then crop
                            let data_slice: &[u8] = &data;
                            let cropped_data = crop_image_data(
                                data_slice,
                                frame.width,
                                frame.height,
                                *x,
                                *y,
                                *width,
                                *height
                            );
                            (cropped_data, *width, *height)
                        }
                    };

                    match
                        png_encoder.write_image(
                            &final_data,
                            final_width,
                            final_height,
                            image::ColorType::Rgba8.into()
                        )
                    {
                        Ok(_) => println!("Screenshot saved as: {:?}", file_name),
                        Err(e) => eprintln!("Failed to encode PNG: {:?}", e),
                    }
                }
                Err(e) => {
                    eprintln!("Failed to access frame data: {:?}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to create file {:?}: {:?}", file_name, e);
        }
    }
}

fn crop_image_data(
    data: &[u8],
    image_width: u32,
    image_height: u32,
    x: u32,
    y: u32,
    width: u32,
    height: u32
) -> Vec<u8> {
    let bytes_per_pixel = 4; // RGBA8
    let mut cropped_data = Vec::new();

    for row in y..(y + height).min(image_height) {
        let row_start = (row * image_width * bytes_per_pixel) as usize;
        let crop_start = row_start + ((x * bytes_per_pixel) as usize);
        let crop_end =
            crop_start +
            ((width * bytes_per_pixel).min((image_width - x) * bytes_per_pixel) as usize);

        if crop_end <= data.len() {
            cropped_data.extend_from_slice(&data[crop_start..crop_end]);
        }
    }

    cropped_data
}

/// Save a Bevy Image to a PNG file
pub fn save_image_as_png(image: &Image, path: &PathBuf) -> Result<(), String> {
    if let Some(data) = &image.data {
        // Ensure the parent directory exists
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| format!("Failed to create directory: {:?}", e))?;
        }

        let file = std::fs::File::create(path)
            .map_err(|e| format!("Failed to create file {:?}: {:?}", path, e))?;
        
        let png_encoder = PngEncoder::new(file);
        
        let width = image.texture_descriptor.size.width;
        let height = image.texture_descriptor.size.height;
        
        png_encoder.write_image(
            data,
            width,
            height,
            image::ColorType::Rgba8.into()
        ).map_err(|e| format!("Failed to encode PNG: {:?}", e))?;
        
        println!("Screenshot saved as: {:?}", path);
        Ok(())
    } else {
        Err("Image data is None".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::app::App;

    #[test]
    fn test_screenshot_plugin_builds() {
        let mut app = App::new();
        app.add_plugins(ScreenshotPlugin);
        assert!(app.world().contains_resource::<ScreenshotManager>());
    }

    #[test]
    fn test_screenshot_manager_default() {
        let manager = ScreenshotManager::default();
        // Initially, runtime should be None
        assert!(manager.runtime.is_none());
        assert!(manager.sender.is_none());
    }

    #[test]
    fn test_screenshot_event_default() {
        let event = ScreenshotEvent::default();
        assert!(matches!(event.capture_type, CaptureType::FullScreen));
        assert!(matches!(event.mouse_overlay, MouseOverlay::Include));
        assert!(event.output_path.is_none());
    }

    #[test]
    fn test_screenshot_event_region() {
        let event = ScreenshotEvent::region(100, 200, 800, 600);
        assert!(
            matches!(event.capture_type, CaptureType::Region {
                x: 100,
                y: 200,
                width: 800,
                height: 600,
            })
        );
    }

    #[test]
    fn test_screenshot_event_builder() {
        let path = PathBuf::from("/tmp/test.png");
        let event = ScreenshotEvent::full_screen()
            .with_mouse_overlay(MouseOverlay::Exclude)
            .with_output_path(path.clone());

        assert!(matches!(event.mouse_overlay, MouseOverlay::Exclude));
        assert_eq!(event.output_path, Some(path));
    }
}