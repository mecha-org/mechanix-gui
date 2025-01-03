use mctk_core::reexports::smithay_client_toolkit::reexports::calloop::channel::Sender;
use std::{
    io::{Error, ErrorKind},
    time::Duration,
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
    time::timeout as tout,
};

use crate::AppMessage;

pub struct NetworkingHandle {
    app_channel: Sender<AppMessage>,
}

impl NetworkingHandle {
    pub fn new(app_channel: Sender<AppMessage>) -> Self {
        NetworkingHandle { app_channel }
    }

    pub async fn run(&self) {
        let mut interval = tokio::time::interval(Duration::from_secs(5));
        loop {
            interval.tick().await; // Ensures consistent periodicity
                                   // println!("NetworkingHandle::run()");
            let is_online = Online::check(250).await.is_ok();
            // println!("NetworkingHandle::run() is_online() {:?}", is_online);
            let _ = &self.app_channel.send(AppMessage::Net { online: is_online });
        }
    }
}

struct Online;
impl Online {
    pub async fn check(timeout: u64) -> Result<(), Error> {
        let dur = Self::parse_timeout(timeout)?;
        let g_res = Self::check_http("clients3.google.com", 80, "/", dur).await;
        // println!("Google HTTP check res {:?}", g_res);
        if g_res.is_ok() {
            return Ok(());
        }

        let f_res = Self::check_http("detectportal.firefox.com", 80, "/", dur).await;
        // println!("Firefox HTTP check res {:?}", f_res);
        if f_res.is_ok() {
            return Ok(());
        }

        Err(Error::new(ErrorKind::Other, "No internet access"))
    }

    pub async fn check_http(host: &str, port: u16, path: &str, dur: Duration) -> Result<(), Error> {
        // Create the HTTP GET request manually
        let request = format!(
            "GET {} HTTP/1.1\r\nHost: {}\r\nConnection: close\r\n\r\n",
            path, host
        );

        // Establish a TCP connection with timeout
        let stream = tout(dur, TcpStream::connect((host, port))).await??;

        // Send the request and read the response
        let mut stream = stream;
        stream.write_all(request.as_bytes()).await?;
        stream.flush().await?;

        let mut buffer = [0u8; 1024];
        let n = stream.read(&mut buffer).await?;

        // Check if the response starts with "HTTP/1.1 200" or similar
        let response = String::from_utf8_lossy(&buffer[..n]);
        if response.starts_with("HTTP/1.1 200") || response.starts_with("HTTP/1.1 204") {
            // println!("HTTP response: {}", &response[..response.len().min(100)]); // Log a snippet
            Ok(())
        } else {
            Err(Error::new(
                ErrorKind::Other,
                format!("Unexpected HTTP response: {}", response),
            ))
        }
    }

    pub fn parse_timeout(timeout: u64) -> Result<Duration, Error> {
        if timeout == 0 {
            Err(Error::new(
                ErrorKind::InvalidInput,
                "cannot set a 0 duration timeout",
            ))
        } else {
            Ok(Duration::from_millis(timeout))
        }
    }
}
