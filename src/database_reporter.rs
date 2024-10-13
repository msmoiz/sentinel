use std::sync::Mutex;

use crate::database::Database;

/// A metric reporter that forwards metrics to a local metrics database.
#[derive(Debug)]
pub struct DatabaseReporter {
    database: Mutex<Database>,
}

impl DatabaseReporter {
    /// Creates a new reporter.
    pub fn from_database(database: Database) -> Self {
        Self {
            database: Mutex::new(database),
        }
    }
}

impl crate::metrics::Reporter for DatabaseReporter {
    fn report(&self, metric: crate::metrics::metric::Metric) {
        let mut database = self.database.lock().unwrap();
        database.put_metric(metric.name, metric.value, metric.time);
    }
}
