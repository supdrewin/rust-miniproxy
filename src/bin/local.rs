use async_std::task;

use miniproxy::{
    cli::Cli,
    config::{Config, LocalConfig},
    daemon::set_daemon,
    local::run_local,
    Result,
};

fn main() -> Result<()> {
    env_logger::init();

    let cli = Cli::new();
    let config = LocalConfig::load_or_default(cli.config.as_deref())?;

    cli.daemon.then(|| set_daemon("minilocal"));
    config.save("config/local.json")?;

    Ok(task::block_on(run_local(config))?)
}
