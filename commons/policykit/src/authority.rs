use zbus::proxy;

use crate::types::{Identity, Subject};
#[proxy(
    interface = "org.freedesktop.PolicyKit1.Authority",
    default_service = "org.freedesktop.PolicyKit1",
    default_path = "/org/freedesktop/PolicyKit1/Authority"
)]
trait Authority {
    /// AuthenticationAgentResponse2 method
    fn authentication_agent_response2(
        &self,
        uid: u32,
        cookie: &str,
        identity: Identity,
    ) -> zbus::Result<()>;

    /// CancelCheckAuthorization method
    fn cancel_check_authorization(&self, cancellation_id: &str) -> zbus::Result<()>;

    /// RegisterAuthenticationAgent method
    fn register_authentication_agent(
        &self,
        subject: Subject<'_>,
        locale: &str,
        object_path: &str,
    ) -> zbus::Result<()>;

    /// RegisterAuthenticationAgentWithOptions method
    fn register_authentication_agent_with_options(
        &self,
        subject: &(
            &str,
            std::collections::HashMap<&str, &zbus::zvariant::Value<'_>>,
        ),
        locale: &str,
        object_path: &str,
        options: std::collections::HashMap<&str, &zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<()>;

    /// UnregisterAuthenticationAgent method
    fn unregister_authentication_agent(
        &self,
        subject: &(
            &str,
            std::collections::HashMap<&str, &zbus::zvariant::Value<'_>>,
        ),
        object_path: &str,
    ) -> zbus::Result<()>;
}
