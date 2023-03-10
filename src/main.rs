use std::{process::{Command}, env};
use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct MyError(String);

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for MyError {}

impl From<Box<dyn Error>> for MyError {
    fn from(err: Box<dyn Error>) -> Self {
        MyError(err.to_string())
    }
}





#[derive(Debug)]
struct NetworkRow {
    name: String,
    signal: String,
}


fn main() {
    let device = env::args().skip(1).next().unwrap_or("wlan0".to_string());
    let output = Command::new("iwctl")
        .args(&["station", &device , "get-networks"])
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

    for network_row in &network_rows {
        println!("Name: {} Signal: {}", network_row.name, network_row.signal);
    }
    

}


