use sysinfo::Disks;
use serde_json::{Value, json};
use std::ffi::OsStr;

use crate::disk::name::disk_name_null_helper;
use crate::disk::formatting::{calculate_disk_percentage_full, convert_disk_space_to_human_readable};
use crate::disk::bytes_convert::bytes_to_gb;

/// List all drives on the system
pub fn list_drives() -> Vec<Value> {
    let disks = Disks::new_with_refreshed_list();
    disks.list().iter().map(|disk| {
        let available_space = convert_disk_space_to_human_readable(bytes_to_gb(disk.available_space()));
        let total_space = convert_disk_space_to_human_readable(bytes_to_gb(disk.total_space()));
        json!({
            "name": disk_name_null_helper(disk.name()),
            "available_space": available_space,
            "total_space": total_space,
            "percentage_full": calculate_disk_percentage_full(disk),
            "letter": disk.mount_point().to_string_lossy().replace(":\\", "")
        })
    }).collect()
}