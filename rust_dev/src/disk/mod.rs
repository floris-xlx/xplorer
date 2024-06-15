pub mod bytes_convert;
pub mod name;
pub mod formatting;
pub mod info;
pub mod list_files;

use crate::disk::info::list_drives;


#[derive(Debug)]
pub struct DisksInfo {
    pub name: String,
    pub available_space: String,
    pub total_space: String,
    pub percentage_full: f64,
    pub letter: String,
}

impl DisksInfo {
    pub fn new(
        name: String,
        available_space: String,
        total_space: String,
        percentage_full: f64,
        letter: String,
    ) -> DisksInfo {

        DisksInfo {
            name,
            available_space,
            total_space,
            percentage_full,
            letter
        }
    }

    /// List all drives on the system and return a vector of DisksInfo objects
    pub fn list_drives() -> Vec<DisksInfo> {
        let drives: Vec<DisksInfo> = list_drives()
            .iter()
            .map(|drive| {
                DisksInfo::new(
                    drive["name"].as_str().unwrap().to_string(),
                    drive["available_space"].as_str().unwrap().to_string(),
                    drive["total_space"].as_str().unwrap().to_string(),
                    drive["percentage_full"].as_f64().unwrap(),
                    drive["letter"].as_str().unwrap().to_string(),
                )
            })
            .collect();

        drives
    }
}
