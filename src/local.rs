use std::str;

use async_std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    prelude::*,
};

use futures::future::FutureExt;
use log::{error, info};

use crate::{
    ciper::CiperTcpStream, config::LocalConfig, pac::serve_pac_file, password::decode_password,
    socks5::req_socks5, spawn_and_log_err, Result,
};

pub async fn run_local(config: LocalConfig) -> Result<()> {
    let host = config.host.unwrap();
    let port = config.port.unwrap();
    let server = config.server.unwrap();
    let password = config.password.unwrap();

    let addr = format!("{host}:{port}");

    info!("MINILOCAL listening on {addr}");
    info!("Serve [ HTTP | HTTPS | SOCKS5 ]");
    info!("PAC url http://{addr}/pac");

    let listener = TcpListener::bind(addr).await?;
    let password = decode_password(&password)?;

    while let Some(stream) = listener.incoming().next().await {
        spawn_and_log_err(serve_conn(
            CiperTcpStream::new(TcpStream::connect(server.clone()).await?, password.clone()),
            stream?,
        ));
    }

    Ok(())
}

async fn serve_conn<T>(mut server_stream: T, mut stream: TcpStream) -> Result<()>
where
    T: Read + Write + Unpin,
    for<'a> &'a T: Read + Write + Unpin,
{
    let mut buf = vec![0_u8; 1024];
    let n = stream.read(&mut buf).await?;

    // info!("{}", String::from_utf8(buf.clone())?);

    let mut headers = [httparse::EMPTY_HEADER; 16];
    let mut req = httparse::Request::new(&mut headers);

    match req.parse(&buf[0..n]) {
        Ok(_) => {
            let host = match req.headers.iter().find(|h| h.name.eq("Host")) {
                Some(h) => str::from_utf8(h.value)?,
                None => return Ok(error!("invalid request")),
            };

            // Serve pac file
            if let Some(path) = req.path {
                if path == "/pac" {
                    return serve_pac_file(stream).await;
                }
            }

            // Do nothing
            if host.contains("127.0.0.1") {
                return Ok(());
            }

            info!("{host}");

            server_stream = req_socks5(server_stream, host).await?;

            match req.method {
                Some("CONNECT") => {
                    stream
                        .write_all(b"HTTP/1.1 200 Tunnel established\r\n\r\n")
                        .await?
                }
                _ => server_stream.write_all(&buf[..n]).await?,
            }
        }
        // 解析失败 则直接理解为socks5代理
        Err(_) => {
            server_stream.write_all(&buf[..n]).await?;
        }
    }

    let (lr, lw) = &mut (&stream, &stream);
    let (tr, tw) = &mut (&server_stream, &server_stream);

    let copy_a = async_std::io::copy(lr, tw);
    let copy_b = async_std::io::copy(tr, lw);

    let _r = futures::select! {
        r1 = copy_a.fuse() => r1,
        r2 = copy_b.fuse() => r2
    };

    Ok(())
}
