mod database;
mod database_reporter;
mod log;
mod metrics;
mod monitor;

use ::log::error;
use database::Database;
use database_reporter::DatabaseReporter;

use crate::monitor::monitor;
use std::thread;

fn main() {
    log::init();

    let database = Database::start();
    let reporter = DatabaseReporter::from_database(database);
    let _guard = metrics::reporter::set_reporter(reporter);

    let monitor_handle = thread::spawn(|| {
        monitor();
    });

    if let Err(e) = monitor_handle.join() {
        error!("monitor error: {e:?}")
    }
}
