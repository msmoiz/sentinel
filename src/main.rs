mod database;
mod log;
mod metrics;
mod monitor;

use ::log::error;

use crate::monitor::monitor;
use std::thread;

fn main() {
    log::init();

    let monitor_handle = thread::spawn(|| {
        monitor();
    });

    if let Err(e) = monitor_handle.join() {
        error!("monitor error: {e:?}")
    }
}
