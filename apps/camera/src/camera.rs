use std::sync::Mutex;

use image::{GenericImageView, ImageBuffer};
use lazy_static::lazy_static;
use mctk_camera::{
    camera::GstCamera,
    types::{CameraFormat, FrameFormat},
};
use mctk_core::{context::Context, reexports::femtovg::rgb::FromSlice};
use mctk_macros::Model;
use rgb::Rgb;

lazy_static! {
    pub static ref RUNTIME: tokio::runtime::Runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();
    pub static ref CAMERA: Camera = Camera {
        frame_buffer: Context::new(ImageBuffer::default()),
        is_initialized: Context::new(false),
        fps: Context::new(30),
        device_index: Context::new(0),
        height: Context::new(480),
        width: Context::new(640),
    };
    pub static ref GST_CAMERA: Mutex<Option<GstCamera>> = Mutex::new(None);
}

#[derive(Model)]
pub struct Camera {
    is_initialized: Context<bool>,
    pub frame_buffer: Context<ImageBuffer<image::Rgb<u8>, Vec<u8>>>,
    pub fps: Context<u32>,
    pub device_index: Context<usize>,
    pub width: Context<u32>,
    pub height: Context<u32>,
}

impl Camera {
    pub fn get() -> &'static Self {
        &CAMERA
    }

    pub fn init() {
        if !*CAMERA.is_initialized.get() {
            println!("initializing camera");
            let camera = match GstCamera::new(
                *CAMERA.device_index.get(),
                Some(CameraFormat::new_from(
                    *CAMERA.width.get(),
                    *CAMERA.height.get(),
                    FrameFormat::MJPEG,
                    *CAMERA.fps.get(),
                )),
            ) {
                Ok(mut c) => {
                    match c.open_stream() {
                        Ok(()) => {
                            println!("camera open success");
                            println!(
                                "camera format: {:?}",
                                CameraFormat::new_from(
                                    *CAMERA.width.get(),
                                    *CAMERA.height.get(),
                                    FrameFormat::MJPEG,
                                    *CAMERA.fps.get(),
                                )
                            );
                        }
                        Err(err) => {
                            println!("failed to open camera stream: {}", err);
                        }
                    };
                    Some(c)
                }
                Err(e) => {
                    println!("failed to create camera, err - {:?}", e);
                    None
                }
            };
            *GST_CAMERA.lock().unwrap() = camera;
            CAMERA.is_initialized.set(true);
        }
    }

    pub fn get_buffer() -> Box<[Rgb<u8>]> {
        Box::from(Self::get().frame_buffer.get().as_rgb())
    }

    pub fn get_height() -> u32 {
        Self::get().frame_buffer.get().height()
    }

    pub fn get_width() -> u32 {
        Self::get().frame_buffer.get().width()
    }

    pub fn save_frame() {
        let frame = Self::get().frame_buffer.get();
        frame.save("frame.jpg").unwrap();

        RUNTIME.spawn(async {
            GST_CAMERA
                .lock()
                .unwrap()
                .as_mut()
                .unwrap()
                .stop_stream()
                .unwrap();

            let camera = match GstCamera::new(
                *CAMERA.device_index.get(),
                Some(CameraFormat::new_from(1280, 720, FrameFormat::MJPEG, 30)),
            ) {
                Ok(mut c) => {
                    match c.open_stream() {
                        Ok(()) => {
                            println!("camera open success");
                            println!(
                                "camera format: {:?}",
                                CameraFormat::new_from(1280, 720, FrameFormat::MJPEG, 30)
                            );
                        }
                        Err(err) => {
                            println!("failed to open camera stream: {}", err);
                        }
                    };
                    Some(c)
                }
                Err(e) => {
                    println!("failed to create camera, err - {:?}", e);
                    None
                }
            };
            let mut camera = camera.unwrap();
            loop {
                if let Ok(frame) = camera.frame() {
                    frame.save("frame2.jpg").unwrap();
                    break;
                }
            }
            camera.stop_stream().unwrap();
            GST_CAMERA
                .lock()
                .unwrap()
                .as_mut()
                .unwrap()
                .open_stream()
                .unwrap();
        });
    }

    pub fn start_fetching() {
        RUNTIME.spawn(async move {
            loop {
                Camera::get();
                match GST_CAMERA.lock().unwrap().as_mut().unwrap().frame() {
                    Ok(f) => {
                        println!("got frame!");
                        Self::get().frame_buffer.set(f);
                    }
                    Err(e) => {
                        println!("error from frame {:?}", e);
                    }
                };
                std::thread::sleep(std::time::Duration::from_secs_f64(
                    1.0 / (*Self::get().fps.get() as f64),
                ));
            }
        });
    }
}
