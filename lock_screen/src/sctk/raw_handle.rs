use raw_window_handle::{
    HasRawDisplayHandle, HasRawWindowHandle, RawDisplayHandle, RawWindowHandle,
};
pub struct RawWaylandHandle(pub RawDisplayHandle, pub RawWindowHandle);

unsafe impl HasRawDisplayHandle for RawWaylandHandle {
    fn raw_display_handle(&self) -> RawDisplayHandle {
        self.0
    }
}

unsafe impl HasRawWindowHandle for RawWaylandHandle {
    fn raw_window_handle(&self) -> RawWindowHandle {
        self.1
    }
}
