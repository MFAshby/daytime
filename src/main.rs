use std::error::{Error};
use std::sync::{Arc};
use std::fmt::{Debug};

use async_std::prelude::*;
use async_std::net::{TcpListener, TcpStream, UdpSocket, SocketAddr};
use async_std::task;

use futures::join;

use chrono::prelude::*;

use log::debug;

use argh::{FromArgs};

/// Format the local time as a string then turn it to bytes
fn now_str_bytes() -> Vec<u8> {
    let now = Local::now();
    format!("{}\n", now).as_bytes().to_vec()
}

/// Log to the debug logger if the Result is an Err variant
fn debug_log<T, E: Debug>(r:  Result<T, E>) {
    if let Err(e) = r {
        debug!("{:?}", e);
    }
}

/// Process TCP client connection
async fn tcp_handle_client(mut tcp_stream: TcpStream) -> Result<(), Box<dyn Error>> {
    let now = now_str_bytes();
    tcp_stream.write(&now).await?;
    Ok(())
}

/// Run TCP server
async fn tcp_daytime_server(address: &str) -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind(address).await?;
    let mut incoming = listener.incoming();
    while let Some(stream) = incoming.next().await {
        let stream = stream?;
        task::spawn(async move {
            debug_log(tcp_handle_client(stream).await);
        });
    }
    Ok(())
}

/// Process UDP client connection
async fn udp_handle_client(socket: &UdpSocket, addr: SocketAddr) -> Result<(), Box<dyn Error>> {
    let now = now_str_bytes();
    socket.send_to(&now, addr).await?;
    Ok(())
}

/// Run UDP server
async fn udp_daytime_server(address: &str) -> Result<(), Box<dyn Error>> {
    let s = UdpSocket::bind(address).await?;
    let socket = Arc::new(s);
    let mut buf = vec![0_u8; 0];
    while let Ok(stream) = socket.recv_from(&mut buf).await {
        let (_, source) = stream;
        let s2 = socket.clone();
        task::spawn(async move {
            debug_log(udp_handle_client(&s2, source).await);
        });
    }
    Ok(())
}

fn default_address() -> String {
    String::from("127.0.0.1:13")
}

#[derive(FromArgs)]
/// Run a daytime server
struct Args {
    #[argh(option, default = "default_address()")]
    /// the local address to bind to (both TCP and UDP)
    address: String,
}

#[async_std::main]
async fn main() {
    env_logger::init();
    let args: Args = argh::from_env();
    let (r1, r2) = join!(tcp_daytime_server(&args.address), udp_daytime_server(&args.address));
    debug_log(r1);
    debug_log(r2);
}
