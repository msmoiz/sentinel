mod log;
mod metrics;

use std::{thread, time::Duration};

use ::log::{debug, error};
use sysinfo::{Disks, System};

fn main() {
    log::init();

    let monitor_handle = thread::spawn(|| {
        monitor();
    });

    if let Err(e) = monitor_handle.join() {
        error!("monitor error: {e:?}")
    }
}

fn monitor() {
    let mut system = System::new();
    let mut disks = Disks::new_with_refreshed_list();
    loop {
        system.refresh_cpu_usage();
        system.refresh_memory();
        let cpu_usage = system.global_cpu_usage();
        let mem_usage = system.used_memory() as f64 / system.total_memory() as f64;
        disks.refresh();
        let disk_usage = {
            let (available_space, total_space) = disks.list().iter().fold((0, 0), |acc, disk| {
                (acc.0 + disk.available_space(), acc.1 + disk.total_space())
            });
            let used_space = total_space - available_space;
            used_space as f64 / total_space as f64
        };

        metric!("cpu_usage", cpu_usage as f64);
        metric!("mem_usage", mem_usage as f64);
        metric!("disk_usage", disk_usage as f64);

        debug!("[monitor] cpu usage: {cpu_usage:.2}%, mem usage: {mem_usage:.2}%, disk usage: {disk_usage:.2}%");
        thread::sleep(Duration::from_secs(1));
    }
}
