mod database;
mod database_reporter;
mod log;
mod metrics;
mod monitor;
mod server;

use ::log::error;
use database::Database;
use database_reporter::DatabaseReporter;

use crate::monitor::monitor;
use std::{
    sync::{Arc, Mutex},
    thread,
};

fn main() {
    log::init();

    let database = Arc::new(Mutex::new(Database::start()));
    let reporter = DatabaseReporter::from_database(database.clone());
    let _guard = metrics::reporter::set_reporter(reporter);

    thread::spawn(|| {
        server::start(database);
    });

    let monitor_handle = thread::spawn(|| {
        monitor();
    });

    if let Err(e) = monitor_handle.join() {
        error!("monitor error: {e:?}")
    }
}
