use sysinfo::Disk;
use crate::disk::bytes_convert::{ bytes_to_gb, gb_to_tb };

pub fn calculate_disk_percentage_full(disk: &Disk) -> f64 {
    let available_gb = bytes_to_gb(disk.available_space());
    let total_gb = bytes_to_gb(disk.total_space());
    let percentage_free = (available_gb / total_gb) * 100.0;
    (100.0 - percentage_free).round() / 100.0
}


pub fn convert_disk_space_to_human_readable(space_gb: f64) -> String {
    if space_gb > 1000.0 {
        format!("{:.2} TB", gb_to_tb(space_gb))
    } else {
        format!("{:.2} GB", space_gb)
    }
}
