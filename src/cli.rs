use clap::{ArgAction::SetTrue, Parser};

#[derive(Parser)]
#[command(version)]
pub struct Cli {
    #[arg(short, long, value_name = "PATH")]
    pub config: Option<String>,
    #[arg(short, long, action = SetTrue)]
    pub daemon: bool,
}

impl Cli {
    pub fn new() -> Self {
        Self::parse()
    }
}
