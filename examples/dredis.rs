use anyhow::Result;
use std::io::ErrorKind::WouldBlock;
use std::net::SocketAddr;
use tokio::io::AsyncWriteExt;
use tokio::net::{TcpListener, TcpStream};
use tracing::{info, warn};

const BUF_SIZE: usize = 4096;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let addr = "0.0.0.0:6379";
    let listener = TcpListener::bind(addr).await?;
    info!("Listening on {}", addr);

    loop {
        let (stream, raddr) = listener.accept().await?;
        info!("Accepted connection from {}", raddr);

        tokio::spawn(async move {
            if let Err(e) = process_redis_comm(stream, raddr).await {
                warn!("Error processing connection from {}: {}", raddr, e);
            }
        });
    }
}

async fn process_redis_comm(mut stream: TcpStream, addr: SocketAddr) -> Result<()> {
    loop {
        let mut buffer = Vec::with_capacity(BUF_SIZE);
        stream.readable().await?;

        match stream.try_read_buf(&mut buffer) {
            Ok(0) => break,
            Ok(n) => {
                let line = String::from_utf8_lossy(&buffer[..n]);
                info!("Received request from {}: {}", addr, line);
                stream.write_all(b"+OK\r\n").await?
            }
            Err(ref e) if e.kind() == WouldBlock => continue,
            Err(e) => return Err(e.into()),
        }
    }
    warn!("Connection closed: {}", addr);
    Ok(())
}
