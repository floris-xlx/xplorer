use std::ffi::OsStr;

pub fn disk_name_null_helper(disk_name: &OsStr) -> String {
    if disk_name.is_empty() {
        "Disk".to_string()
    } else {
        // remove the "" from the disk name
        disk_name.to_os_string().into_string().unwrap()
    }
}
