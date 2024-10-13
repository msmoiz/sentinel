use std::{
    collections::HashMap,
    fs::{self, File},
    io::Write,
    path::PathBuf,
    thread::{self},
    time::Duration,
};

use anyhow::Context;
use chrono::{DateTime, Days, Utc};

/// A time-series database.
#[derive(Debug)]
pub struct Database {
    /// A map of metric names to active storage files.
    metric_to_storage: HashMap<String, File>,
}

impl Database {
    /// Starts a new database instance.
    pub fn start() -> Self {
        thread::spawn(|| Self::clean());

        Self {
            metric_to_storage: HashMap::new(),
        }
    }

    /// Stores a metric datapoint.
    pub fn put_metric(&mut self, name: String, value: f64, time: DateTime<Utc>) {
        let storage = self
            .metric_to_storage
            .entry(name.clone())
            .or_insert_with(|| Self::create_storage_file(name.clone()).unwrap());

        let metric = Metric { value, time };
        storage.write_all(&metric.to_bytes()).unwrap();

        const ONE_MB_IN_BYTES: u64 = 1024 * 1024;
        if storage.metadata().unwrap().len() > ONE_MB_IN_BYTES {
            let new_storage = Self::create_storage_file(name.clone()).unwrap();
            self.metric_to_storage.insert(name, new_storage);
        }
    }

    /// Creates a new storage file for the provided metric.
    ///
    /// Each metric has its own set of storage files. Returns an open file
    /// descriptor for the file in append mode.
    fn create_storage_file(metric_name: String) -> anyhow::Result<File> {
        let storage_dir = PathBuf::from("database").join(metric_name);

        fs::create_dir_all(&storage_dir)
            .with_context(|| format!("failed to create metric dir {}", storage_dir.display()))?;

        let current_time = Utc::now().format("%Y%m%d%H%M").to_string();

        let storage_path = storage_dir.join(current_time);

        let storage_file = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&storage_path)
            .with_context(|| format!("failed to create metric file {}", storage_path.display()))?;

        Ok(storage_file)
    }

    /// Cleans up files older than one week.
    fn clean() {
        loop {
            for dir in fs::read_dir("database")
                .unwrap()
                .filter_map(Result::ok)
                .map(|entry| entry.path())
                .filter(|entry| entry.is_dir())
            {
                for file in dir
                    .read_dir()
                    .unwrap()
                    .filter_map(Result::ok)
                    .filter(|entry| {
                        let mod_time: DateTime<Utc> =
                            entry.metadata().unwrap().modified().unwrap().into();
                        let cutoff = Utc::now().checked_sub_days(Days::new(7)).unwrap();
                        mod_time < cutoff
                    })
                    .map(|entry| entry.path())
                {
                    fs::remove_file(file).unwrap();
                }
            }

            thread::sleep(Duration::from_secs(3600));
        }
    }
}

/// An internal representation of a metric.
struct Metric {
    /// The value of the metric.
    value: f64,
    /// The timestamp associated with the metric.
    time: DateTime<Utc>,
}

impl Metric {
    /// Serializes this object to bytes.
    fn to_bytes(&self) -> Vec<u8> {
        let value = self.value.to_be_bytes();
        let time = (self.time.timestamp() as u64).to_be_bytes();
        value.into_iter().chain(time.into_iter()).collect()
    }
}
