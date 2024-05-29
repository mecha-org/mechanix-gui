use zbus::proxy;
#[proxy(
    interface = "org.freedesktop.login1.User",
    default_service = "org.freedesktop.login1",
    default_path = "/org/freedesktop/login1/user/self"
)]
trait User {
    /// Sessions property
    #[zbus(property)]
    fn sessions(&self) -> zbus::Result<Vec<(String, zbus::zvariant::OwnedObjectPath)>>;
}
