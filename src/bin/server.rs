use async_std::task;
use log::error;

use miniproxy::{cli::Cli, config::ServerConfig, daemon::set_daemon, server::run_server};

const SERVER_NAME: &'static str = "miniserver";

fn main() {
    env_logger::init();

    let cli = Cli::new();

    let mut config = match cli.config {
        Some(path) => ServerConfig::load_from_file(&path).unwrap(),
        None => ServerConfig::default(),
    };

    if let Some(host) = cli.host {
        config.host = Some(host);
    }

    if let Some(port) = cli.port {
        config.port = match port.parse::<u16>() {
            Ok(port) => Some(port),
            Err(e) => {
                return error!("invalid port {e:?}");
            }
        }
    }

    if let Some(password) = cli.password {
        config.password = Some(password);
    }

    if cli.daemon {
        set_daemon(SERVER_NAME);
    }

    task::block_on(run_server(config)).unwrap();
}
