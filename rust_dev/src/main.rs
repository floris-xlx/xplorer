// // Prevents additional console window on Windows in release, DO NOT REMOVE!!
// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// pub mod disk;

// use crate::disk::DisksInfo;

// fn main() {
//     println!("Hello, world!");

//     // let drives: Vec<DisksInfo> = DisksInfo::list_drives();

//     // println!("{drives:#?}");
// }

use cronitor::{ cronitor, cron_runtime };


#[cronitor("*/1 * * * *")]
fn ping_every_minute() {
    println!("Ping! Pong!");
}


#[cronitor("*/15 * * * *")]
fn ping_every_15minute() {
    println!("Ping! Pong! 15 minutes");
}


#[cronitor("0 0 1 * *")]
fn ping_every_day_at_6() {
    println!("Ping! Pong! 6 am!");
}




fn main() {
    cron_runtime();

    loop {
        std::thread::park();
    }
}
