use clap::{ArgAction::SetTrue, Parser};

#[derive(Parser)]
#[command(version)]
pub struct Cli {
    #[arg(short, long, value_name = "HOST")]
    pub host: Option<String>,
    #[arg(short, long, value_name = "SERVER")]
    pub server: Option<String>,
    #[arg(short, long, value_name = "PORT")]
    pub port: Option<String>,
    #[arg(short('P'), long, value_name = "PASSWORD")]
    pub password: Option<String>,
    #[arg(short, long, value_name = "CONFIG")]
    pub config: Option<String>,
    #[arg(short, long, action = SetTrue)]
    pub daemon: bool,
}

impl Cli {
    pub fn new() -> Self {
        Self::parse()
    }
}
