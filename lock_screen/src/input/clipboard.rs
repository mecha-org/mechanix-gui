use std::ffi::c_void;

use iced_wgpu::core::Clipboard;

pub struct WaylandClipboard(smithay_clipboard::Clipboard);

impl WaylandClipboard {
    /// Creates a new clipboard which will be running on its own thread with its own
    /// event queue to handle clipboard requests.
    ///
    /// # Safety
    /// Display must be a valid *mut wl_display pointer, and it must remain valid for
    /// as long as the Clipboard object is alive.
    pub unsafe fn new(display: *mut c_void) -> Self {
        Self(smithay_clipboard::Clipboard::new(display))
    }
}

impl Clipboard for WaylandClipboard {
    fn read(&self) -> Option<String> {
        self.0.load().ok()
    }

    fn write(&mut self, contents: String) {
        self.0.store(contents);
    }
}
