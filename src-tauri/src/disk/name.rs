use std::ffi::OsStr;

pub fn disk_name_null_helper(disk_name: &OsStr) -> String {
    disk_name.to_str().unwrap_or("Disk").to_string()
}
