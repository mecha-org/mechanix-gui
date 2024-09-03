use zbus::proxy;
#[proxy(
    interface = "org.freedesktop.login1.Session",
    default_service = "org.freedesktop.login1",
    default_path = "/org/freedesktop/login1"
)]
trait Session {
    /// Lock method
    fn lock(&self) -> zbus::Result<()>;

    /// Unlock method
    fn unlock(&self) -> zbus::Result<()>;

    /// Set locked hint
    fn set_locked_hint(&self, locked: bool) -> zbus::Result<()>;

    /// Lock signal
    #[zbus(signal)]
    fn lock(&self) -> zbus::Result<()>;

    /// Unlock signal
    #[zbus(signal)]
    fn unlock(&self) -> zbus::Result<()>;

    /// Id property
    #[zbus(property)]
    fn id(&self) -> zbus::Result<String>;

    /// TTY property
    #[zbus(property, name = "TTY")]
    fn tty(&self) -> zbus::Result<String>;

    /// LockedHint property
    #[zbus(property, name = "LockedHint")]
    fn locked_hint(&self) -> zbus::Result<bool>;
}
