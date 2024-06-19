pub fn bytes_to_gb(bytes: u64) -> f64 {
    bytes as f64 / 1024.0 / 1024.0 / 1024.0
}

pub fn gb_to_tb(gb: f64) -> f64 {
    gb / 1024.0
}