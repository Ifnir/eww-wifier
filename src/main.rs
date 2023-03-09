use std::process::{Command};
use std::{str};
use serde::{Serialize, Deserialize};
// use serde_json::{Result};
#[derive(Debug, Deserialize, PartialEq, Eq, Serialize)]
struct Device {
    name: String,
    address: String,
    powered: bool,
    adapter: String,
    mode: String,
}

impl Device {
    fn from_utf8(s: &[u8]) -> Result<Vec<Device>, String> {
        let s = match String::from_utf8(s.to_vec()) {
            Ok(v) => v,
            Err(e) => return Err(format!("Invalid UTF-8 sequence: {}", e)),
        };
        
        let mut lines = s.lines().skip(3); // skip the header lines
        println!("{:#?}", lines);
        let mut devices = Vec::new();
        while let Some(line) = lines.next() {
            let device: Device = match serde_json::from_str(line) {
                Ok(d) => d,
                Err(e) => return Err(format!("Failed to parse device: {}", e)),
            };
            devices.push(device);
        }
        
        Ok(devices)
    }
}

fn main() {
    let output = Command::new("iwctl")
        .args(&["station", "wlan0", "get-networks"])
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let networks: Vec<&str> = stdout.lines().skip(1).map(|line| line.split_whitespace().nth(0).unwrap_or("")).collect();

    println!("Available Wi-Fi networks:");
    for network in networks {
        println!("{}", network);
    }
}


