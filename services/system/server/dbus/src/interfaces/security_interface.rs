use std::ffi::{CStr, CString};

use pam_client::conv_mock::Conversation;
use pam_client::{Context, ConversationHandler, ErrorCode, Flag};
use policykit::{authority::AuthorityProxy, types::Identity};
use zbus::message::Header;
use zbus::names::BusName;
use zbus::proxy::CacheProperties;
use zbus::Connection;
use zbus::{fdo::DBusProxy, fdo::Error as ZbusError, interface};

pub struct SecurityBusInterface {}

#[interface(name = "org.mechanix.services.Security")]
impl SecurityBusInterface {
    pub async fn change_password(
        &self,
        old: String,
        secret: String,
        new: String,
        #[zbus(header)] hdr: Header<'_>,
        #[zbus(connection)] conn: &Connection,
    ) -> Result<bool, ZbusError> {
        let user_info_r = get_user_info(hdr, conn).await;

        if let Err(e) = &user_info_r {
            println!("Error while getting user info {:?}", e);
            return Ok(false);
        }

        let (_, username) = user_info_r.unwrap();

        self.change_user_password(username, old, secret, new)
    }

    pub async fn is_password_set(
        &self,
        #[zbus(header)] hdr: Header<'_>,
        #[zbus(connection)] conn: &Connection,
    ) -> Result<bool, ZbusError> {
        let user_info_r = get_user_info(hdr, conn).await;

        if let Err(e) = &user_info_r {
            println!("Error while getting user info {:?}", e);
            return Ok(false);
        }

        let (_, username) = user_info_r.unwrap();

        self.check_password_set(username).await
    }

    pub async fn authenticate_user(
        &self,
        password: String,
        secret: String,
        #[zbus(header)] hdr: Header<'_>,
        #[zbus(connection)] conn: &zbus::Connection,
    ) -> Result<bool, ZbusError> {
        let user_info_r = get_user_info(hdr, conn).await;

        if let Err(e) = &user_info_r {
            println!("Error while getting user info {:?}", e);
            return Ok(false);
        }

        let (_, username) = user_info_r.unwrap();

        self.authenticate(username, password, secret)
    }

    pub async fn authenticate_polkit_request(
        &self,
        password: String,
        secret: String,
        cookie: String,
        identity: Identity,
        #[zbus(header)] hdr: Header<'_>,
        #[zbus(connection)] conn: &zbus::Connection,
    ) -> Result<bool, ZbusError> {
        println!("SecurityBusInterface::authenticate_polkit_request() ",);

        let user_info_r = get_user_info(hdr, conn).await;

        if let Err(e) = &user_info_r {
            println!("Error while getting user info {:?}", e);
            return Ok(false);
        }

        let (uid, username) = user_info_r.unwrap();

        self.authenticate_polkit(uid, username, password, secret, cookie, identity)
            .await
    }
}

impl SecurityBusInterface {
    pub fn change_user_password(
        &self,
        username: String,
        mut old: String,
        secret: String,
        mut new: String,
    ) -> Result<bool, ZbusError> {
        if old.len() > 0 {
            old = format!("{}{}", old, secret);
        }

        if new.len() > 0 {
            new = format!("{}{}", new, secret);
        }

        let context_r = Context::new(
            "mechanix-shell", // Service name
            Some(&username),
            ChangePasswordConversation::with_credentials(old, new),
        );

        if let Err(e) = context_r {
            println!("Error creating context {:?}", e);
            return Ok(false);
        }

        let mut context = context_r.unwrap();

        let change_password = context.chauthtok(Flag::NONE);
        println!("change_password {:?}", change_password);
        if let Err(e) = &change_password {
            println!("Error while changing password {:?}", e);
            let error_code = e.code();
            println!("Error code {:?}", error_code);
            return Ok(false);
        };

        Ok(true)
    }

    pub async fn check_password_set(&self, username: String) -> Result<bool, ZbusError> {
        let context = Context::new(
            "mechanix-shell", // Service name
            None,
            Conversation::with_credentials(username, "".to_string()),
        );

        if let Err(e) = &context {
            println!("Error creating context {:?}", e);
            return Err(ZbusError::Failed("Entry initialization".to_string()));
        }

        let mut context = context.unwrap();

        let auth_res = context.authenticate(Flag::NONE);

        if let Err(e) = &auth_res {
            println!("Error authenticating {:?}", e);
            let error_code = e.code();
            println!("Error code is {:?}", error_code);

            if error_code == ErrorCode::USER_UNKNOWN {
                return Ok(false);
            }

            return Ok(true);
        }

        Ok(false)
    }

    fn authenticate(
        &self,
        username: String,
        mut password: String,
        secret: String,
    ) -> Result<bool, ZbusError> {
        if secret.is_empty() {
            println!("secret cannot be empty");
            return Ok(false);
        }

        if password.len() > 0 {
            password = format!("{}{}", password, secret);
        }

        let context_r = Context::new(
            "mechanix-shell", // Service name
            None,
            Conversation::with_credentials(username, password),
        );

        if let Err(e) = &context_r {
            println!("Error creating context: {:?}", e);
            return Ok(false);
        }

        let mut context = context_r.unwrap();

        // Authenticate the user
        let auth_r = context.authenticate(Flag::NONE);

        if let Err(e) = &auth_r {
            println!("Error authenticating: {:?}", e);
            return Ok(false);
        }

        Ok(true)
    }

    pub async fn authenticate_polkit(
        &self,
        uid: u32,
        username: String,
        password: String,
        secret: String,
        cookie: String,
        identity: Identity,
    ) -> Result<bool, ZbusError> {
        let auth_res = self.authenticate(username, password, secret);
        if let Err(e) = &auth_res {
            println!("Error in authentication {:?}", e);
            return Err(ZbusError::AuthFailed("Authentication failed".to_string()));
        }

        let auth_success = auth_res.unwrap();

        if !auth_success {
            println!("Authentication failed");
            return Err(ZbusError::AuthFailed("Authentication failed".to_string()));
        }

        let connection = zbus::Connection::system().await.unwrap();
        let authority = AuthorityProxy::new(&connection).await.unwrap();
        println!("before sending agent res {:?} {:?}", uid, cookie);
        if let Err(e) = authority
            .authentication_agent_response2(uid, &cookie, identity)
            .await
        {
            println!("Error while sending agent response {:?}", e);
            // return Err(PolkitError::Failed);
        };
        println!("after sending agent res");

        Ok(true)
    }
}

async fn get_user_info(hdr: Header<'_>, conn: &zbus::Connection) -> anyhow::Result<(u32, String)> {
    let dbus_proxy = DBusProxy::builder(conn)
        .cache_properties(CacheProperties::No)
        .build()
        .await?;
    let uid = dbus_proxy
        .get_connection_credentials(BusName::Unique(hdr.sender().unwrap().to_owned()))
        .await?
        .unix_user_id()
        .unwrap();
    let user = users::get_user_by_uid(uid);
    let username = user.unwrap().name().to_str().unwrap().to_string();
    Ok((uid, username))
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ChangePasswordConversation {
    pub old: String,
    pub new: String,
}

impl ChangePasswordConversation {
    #[must_use]
    pub fn new() -> Self {
        Self {
            old: String::from(""),
            new: String::from(""),
        }
    }

    pub fn with_credentials(old: String, new: String) -> Self {
        Self { old, new }
    }
}

impl Default for ChangePasswordConversation {
    fn default() -> Self {
        Self::new()
    }
}

impl ConversationHandler for ChangePasswordConversation {
    fn init(&mut self, default_user: Option<impl AsRef<str>>) {}

    fn prompt_echo_on(&mut self, msg: &CStr) -> Result<CString, ErrorCode> {
        println!("Conversation::prompt_echo_on() {:?}", msg);
        Ok(CString::new("old: ").unwrap())
    }

    fn prompt_echo_off(&mut self, msg: &CStr) -> Result<CString, ErrorCode> {
        println!("Conversation::prompt_echo_off() {:?}", msg);
        let msg = msg.to_str().unwrap();
        let reply = if msg == "old" {
            self.old.clone()
        } else if msg == "new" {
            self.new.clone()
        } else {
            panic!("invalid msg")
        };

        CString::new(reply).map_err(|_| ErrorCode::CONV_ERR)
    }

    fn text_info(&mut self, _msg: &CStr) {}

    fn error_msg(&mut self, _msg: &CStr) {}

    fn radio_prompt(&mut self, _msg: &CStr) -> Result<bool, ErrorCode> {
        Ok(false)
    }
}
