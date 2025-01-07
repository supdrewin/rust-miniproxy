use async_std::{fs, net::TcpStream, prelude::*};

use log::info;

use crate::Result;

pub async fn serve_pac_file(mut stream: TcpStream) -> Result<()> {
    info!("serve pac file");

    let file_contents = fs::read("proxy.pac").await?;
    let length = file_contents.len();

    stream.write_all(b"HTTP/1.1 200 OK\r\n").await?;
    stream
        .write_all(format!("Content-Length: {length}\r\n").as_bytes())
        .await?;
    stream.write_all(b"Server: minilocal").await?;
    stream.write_all(b"Connection: close").await?;
    stream
        .write_all(b"Content-Type: application/x-ns-proxy-autoconfig\r\n")
        .await?;
    stream.write_all(b"\r\n").await?;
    stream.write_all(&file_contents).await?;

    Ok(())
}
