use async_std::task;

use miniproxy::{
    cli::Cli,
    config::{Config, ServerConfig},
    daemon::set_daemon,
    server::run_server,
    Result,
};

fn main() -> Result<()> {
    env_logger::init();

    let cli = Cli::new();
    let config = ServerConfig::load_or_default(cli.config.as_deref())?;

    cli.daemon.then(|| set_daemon("miniserver"));
    config.save("config/server.json")?;

    Ok(task::block_on(run_server(config))?)
}
