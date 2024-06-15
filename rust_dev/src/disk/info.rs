use sysinfo::Disks;
use serde_json::{Value, json};
use std::ffi::OsStr;

use crate::disk::name::disk_name_null_helper;
use crate::disk::formatting::{calculate_disk_percentage_full, convert_disk_space_to_human_readable};
use crate::disk::bytes_convert::bytes_to_gb;

/// List all drives on the system
pub fn list_drives() -> Vec<Value> {
    // create new Disks object with refreshed list
    let disks: Disks = Disks::new_with_refreshed_list();

    println!("{:#?}", disks.list());

    // open scoped vector to store drive information
    let mut drives: Vec<Value> = Vec::new();

    // iterate over each disk in the list
    for disk in disks.list() {
        let disk_available_space: u64 = disk.available_space();
        let disk_name: &OsStr = disk.name();
        let disk_name_formatted: String = disk_name_null_helper(disk_name);
        let disk_percentage_full: f64 = calculate_disk_percentage_full(&disk);

        let disk_letter: String = disk.mount_point().to_string_lossy().to_string();
        // remove the :// part
        let disk_letter: String = disk_letter.replace(":\\", "");


        let disk_total_space: u64 = disk.total_space();

        let disk_available_space_gb: f64 = bytes_to_gb(disk_available_space);
        let disk_total_space_gb: f64 = bytes_to_gb(disk_total_space);

        let disk_available_space_human_readable: String =
            convert_disk_space_to_human_readable(disk_available_space_gb);
        let disk_total_space_human_readable: String =
            convert_disk_space_to_human_readable(disk_total_space_gb);

        let drive: Value = json!({
            "name": disk_name_formatted,
            "available_space": disk_available_space_human_readable,
            "total_space": disk_total_space_human_readable,
            "percentage_full": disk_percentage_full,
            "letter": disk_letter
        });

        drives.push(drive);
    }

    drives
}
