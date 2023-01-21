use anyhow::Result;
use std::time::Instant;

pub struct Solution {
    pub day_no: u8,
    pub part_1: fn(&str) -> Result<String>,
    pub part_2: fn(&str) -> Result<String>,
}

pub fn not_implemented(_: &str) -> Result<String> {
    Ok("Not implemented yet".to_string())
}

pub fn elapsed_string(now: Instant) -> String {
    if now.elapsed().as_millis() > 2 {
        format!("{}ms", now.elapsed().as_millis())
    } else {
        format!("{}μs", now.elapsed().as_micros())
    }
}
