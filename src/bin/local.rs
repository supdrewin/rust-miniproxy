use async_std::task;
use log::error;

use miniproxy::{cli::Cli, config::LocalConfig, daemon::set_daemon, local::run_local};

const LOCAL_NAME: &'static str = "minilocal";

fn main() {
    env_logger::init();

    let cli = Cli::new();

    let mut config = match cli.config {
        Some(path) => LocalConfig::load_from_file(&path).unwrap(),
        None => LocalConfig::default(),
    };

    if let Some(host) = cli.host {
        config.host = Some(host);
    }

    if let Some(server) = cli.server {
        config.server = Some(server);
    }

    if let Some(port) = cli.port {
        config.port = match port.parse::<u16>() {
            Err(e) => return error!("invalid port {e:?}"),
            Ok(port) => Some(port),
        }
    }

    if let Some(password) = cli.password {
        config.password = Some(password);
    }

    if config.server.is_none() {
        return error!("server address required");
    }

    if config.password.is_none() {
        return error!("password required");
    }

    if cli.daemon {
        set_daemon(LOCAL_NAME);
    }

    task::block_on(run_local(config)).unwrap();
}
