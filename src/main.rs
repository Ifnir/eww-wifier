use std::process::{Command};
use std::{str, env};
//use serde::{Serialize, Deserialize};
// use serde_json::{Result};
struct Device {

}

impl Device {
    fn from_utf8(s: &[u8]) -> Result<String, std::str::Utf8Error> {
        match String::from_utf8(s.to_vec()) {
            Ok(v) => Ok(v),
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        }
    }
}
fn main() {
    let output = Command::new("iwctl").args(["device", "list"]).output().expect("failed to execute process");
    println!("{:?}", output);
}
