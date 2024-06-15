// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod disk;

use crate::disk::DisksInfo;

fn main() {
    println!("Hello, world!");

    let drives: Vec<DisksInfo> = DisksInfo::list_drives();

    println!("{drives:#?}");
}

