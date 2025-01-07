use std::error::Error;

use async_std::{future::Future, task};

pub type Result<T> = std::result::Result<T, Box<dyn Error + Send + Sync>>;

fn spawn_and_log_err<F>(fut: F) -> task::JoinHandle<()>
where
    F: Future<Output = Result<()>> + Send + 'static,
{
    task::spawn(async move {
        if let Err(e) = fut.await {
            log::error!("conn err: {e:?}");
        }
    })
}

pub mod ciper;
pub mod cli;
pub mod config;
pub mod daemon;
pub mod local;
pub mod password;
pub mod server;

mod pac;
mod socks5;
