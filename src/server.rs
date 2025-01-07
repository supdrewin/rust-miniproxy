use async_std::{
    net::{TcpListener, ToSocketAddrs},
    prelude::*,
};

use log::info;

use crate::{
    ciper::CiperTcpStream, config::ServerConfig, password::decode_password, socks5::serve_socks5,
    spawn_and_log_err, Result,
};

pub async fn run_server(config: ServerConfig) -> Result<()> {
    let host = config.host.unwrap();
    let port = config.port.unwrap();
    let password = config.password.unwrap();

    let addr = format!("{host}:{port}");
    info!("server listening on {addr}...");

    serve(addr, decode_password(&password)?).await
}

async fn serve<A: ToSocketAddrs>(addr: A, ciper_password: Vec<u8>) -> Result<()> {
    let server = TcpListener::bind(addr).await?;

    while let Some(stream) = server.incoming().next().await {
        let stream = stream?;
        let stream = CiperTcpStream::new(stream, ciper_password.clone());

        spawn_and_log_err(serve_socks5(stream));
    }

    Ok(())
}
