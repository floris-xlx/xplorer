use sysinfo::Disk;
use crate::disk::bytes_convert::{ bytes_to_gb, gb_to_tb };

pub fn calculate_disk_percentage_full(disk: &Disk) -> f64 {
    let disk_available_space: u64 = disk.available_space();
    let disk_total_space: u64 = disk.total_space();

    let disk_available_space_gb: f64 = bytes_to_gb(disk_available_space);
    let disk_total_space_gb: f64 = bytes_to_gb(disk_total_space);

    let disk_percentage_free: f64 = (disk_available_space_gb / disk_total_space_gb) * 100.0;
    let disk_percentage_full: f64 = 100.0 - disk_percentage_free;
    let disk_percentage_full_rounded: f64 = (disk_percentage_full * 100.0).round() / 100.0;

    disk_percentage_full_rounded
}

pub fn convert_disk_space_to_human_readable(disk_space: f64) -> String {
    if disk_space > 1000.0 {
        let disk_space_tb: f64 = gb_to_tb(disk_space);
        format!("{disk_space_tb:.2} TB")
    } else {
        format!("{disk_space:.2} GB")
    }
}
