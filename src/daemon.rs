// use log::{error, info};

pub fn set_daemon(_name: &str) {
    log::warn!("can't be daemonized on windows yet!");
}
