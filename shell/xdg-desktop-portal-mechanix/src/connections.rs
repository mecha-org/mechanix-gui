use std::collections::HashMap;
use std::future::Future;
use zbus::zvariant;
use futures::future::{abortable, AbortHandle};

const PORTAL_RESPONSE_SUCCESS: u32 = 0;
const PORTAL_RESPONSE_CANCELLED: u32 = 1;
const PORTAL_RESPONSE_OTHER: u32 = 2;


#[derive(zvariant::Type)]
#[zvariant(signature = "(ua{sv})")]
pub enum PortalResponse<T: zvariant::Type + serde::Serialize> {
    Success(T),
    Cancelled,
    Other,
}

impl<T: zvariant::Type + serde::Serialize> serde::Serialize for PortalResponse<T> {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Success(res) => (PORTAL_RESPONSE_SUCCESS, res).serialize(serializer),
            Self::Cancelled => (
                PORTAL_RESPONSE_CANCELLED,
                HashMap::<String, zvariant::Value>::new(),
            )
                .serialize(serializer),
            Self::Other => (
                PORTAL_RESPONSE_OTHER,
                HashMap::<String, zvariant::Value>::new(),
            )
                .serialize(serializer),
        }
    }
}

struct Request(AbortHandle);

#[zbus::interface(name = "org.freedesktop.impl.portal.Request")]
impl Request {
    fn close(&self) {
        self.0.abort();
    }
}

impl Request {
    async fn run<T, DropFut, F, Fut>(
        connection: &zbus::Connection,
        handle: &zvariant::ObjectPath<'_>,
        on_cancel: F,
        task: Fut,
    ) -> PortalResponse<T>
    where
        T: zvariant::Type + serde::Serialize,
        DropFut: Future<Output = ()>,
        F: FnOnce() -> DropFut,
        Fut: Future<Output = PortalResponse<T>>,
    {
        let (abortable, abort_handle) = abortable(task);
        let _ = connection
            .object_server()
            .at(handle, Request(abort_handle))
            .await;
        let resp = abortable.await;
        let _ = connection
            .object_server()
            .remove::<Request, _>(handle)
            .await;
        match resp {
            Ok(resp) => resp,
            _ => {
                on_cancel().await;
                PortalResponse::Cancelled
            }
        }
    }
}

struct Session<T: Send + Sync + 'static> {
    data: T,
    close_cb: Option<Box<dyn FnOnce(&mut T) + Send + Sync + 'static>>,
}

impl<T: Send + Sync + 'static> Session<T> {
    fn new<F: FnOnce(&mut T) + Send + Sync + 'static>(data: T, cb: F) -> Self {
        Self {
            data,
            close_cb: Some(Box::new(cb)),
        }
    }
}

#[zbus::interface(name = "org.freedesktop.impl.portal.Session")]
impl<T: Send + Sync + 'static> Session<T> {
    async fn close(&mut self, #[zbus(signal_context)] signal_ctxt: zbus::SignalContext<'_>) {
        // XXX error?
        let _ = self.closed(&signal_ctxt).await;
        let _ = signal_ctxt
            .connection()
            .object_server()
            .remove::<Self, _>(signal_ctxt.path())
            .await;
        if let Some(cb) = self.close_cb.take() {
            cb(&mut self.data);
        }
    }

    #[zbus(signal)]
    async fn closed(&self, signal_ctxt: &zbus::SignalContext<'_>) -> zbus::Result<()>;

    #[zbus(property, name = "version")]
    fn version(&self) -> u32 {
        1 // XXX?
    }
}

impl<Data: Send + Sync + 'static> std::ops::Deref for Session<Data> {
    type Target = Data;

    fn deref(&self) -> &Data {
        &self.data
    }
}

impl<Data: Send + Sync + 'static> std::ops::DerefMut for Session<Data> {
    fn deref_mut(&mut self) -> &mut Data {
        &mut self.data
    }
}

async fn session_interface<Data: Send + Sync + 'static>(
    connection: &zbus::Connection,
    session_handle: &zvariant::ObjectPath<'_>,
) -> Option<zbus::InterfaceRef<Session<Data>>> {
    connection
        .object_server()
        .interface(session_handle)
        .await
        .ok()
}
