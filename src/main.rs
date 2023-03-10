use std::process::{Command};
/* use std::{str};
use serde::{Serialize, Deserialize}; */
// use serde_json::{Result};
/* #[derive(Debug, Deserialize, PartialEq, Eq, Serialize)]
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
} */

#[derive(Debug)]
struct NetworkRow {
    name: String,
    signal: String,
}


fn main() {
    let output = Command::new("iwctl")
        .args(&["station", "wlan0", "get-networks"])
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8_lossy(&output.stdout);
    
    
    let mut network_rows: Vec<NetworkRow> = Vec::new();
    for line in stdout.lines().skip(3).skip(1) {
        let mut name = line.split_whitespace().take_while(|field| *field != "psk").collect::<Vec<&str>>().join(" ");
        name = name.trim().replace("\u{1b}[1;90m> \u{1b}[0m", "").to_string();
        if !name.is_empty() {
            let asterisk_only = line.split_whitespace().last().unwrap_or("").to_string();
            let signal =  asterisk_only.chars().filter(|c| *c == '*').collect::<String>();
            let network_row = NetworkRow {
                name,
                signal,
            };
            network_rows.push(network_row);
        }
      
    }

    println!("{:?}", network_rows)

}


