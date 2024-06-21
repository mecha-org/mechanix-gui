use std::collections::HashMap;
use zbus::zvariant;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Identity {
    pub identity_kind: String,
    pub identity_details: HashMap<String, zvariant::OwnedValue>,
}

impl zvariant::Type for Identity {
    fn signature() -> zvariant::Signature<'static> {
        unsafe { zvariant::Signature::from_bytes_unchecked(b"(sa{sv})") }
    }
}

#[derive(serde::Serialize)]
pub struct Subject<'a> {
    pub subject_kind: &'a str,
    pub subject_details: HashMap<&'a str, zvariant::Value<'a>>,
}

impl<'a> zvariant::Type for Subject<'a> {
    fn signature() -> zvariant::Signature<'static> {
        unsafe { zvariant::Signature::from_bytes_unchecked(b"(sa{sv})") }
    }
}
