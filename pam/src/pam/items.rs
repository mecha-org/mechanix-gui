#[repr(u32)]
pub enum ItemType {
    Service = 1,

    User = 2,

    Tty = 3,

    RHost = 4,

    Conv = 5,

    AuthTok = 6,

    OldAuthTok = 7,

    RUser = 8,

    UserPrompt = 9,

    FailDelay = 10,

    XDisplay = 11,

    XAuthData = 12,

    AuthTokType = 13,
}

pub trait Item {
    type Raw;

    fn type_id() -> ItemType;

    unsafe fn from_raw(raw: *const Self::Raw) -> Self;

    fn into_raw(self) -> *const Self::Raw;
}

macro_rules! cstr_item {
    ($name:ident) => {
        #[derive(Debug)]
        pub struct $name<'s>(pub &'s std::ffi::CStr);

        impl<'s> std::ops::Deref for $name<'s> {
            type Target = &'s std::ffi::CStr;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl<'s> Item for $name<'s> {
            type Raw = libc::c_char;

            fn type_id() -> ItemType {
                ItemType::$name
            }

            unsafe fn from_raw(raw: *const Self::Raw) -> Self {
                Self(std::ffi::CStr::from_ptr(raw))
            }

            fn into_raw(self) -> *const Self::Raw {
                self.0.as_ptr()
            }
        }
    };
}

cstr_item!(Service);
cstr_item!(User);
cstr_item!(Tty);
cstr_item!(RHost);

cstr_item!(AuthTok);
cstr_item!(OldAuthTok);
cstr_item!(RUser);
cstr_item!(UserPrompt);
