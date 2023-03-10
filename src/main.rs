use std::{process::{Command}, env};

#[derive(Debug)]
struct NetworkRow {
    name: String,
    signal: String,
}


fn parse_stdout(stdout: &str) -> Vec<NetworkRow> {
    let mut network_rows: Vec<NetworkRow> = Vec::new();
    for line in stdout.lines().skip(3).skip(1) {
        let mut name = line.split_whitespace().take_while(|field| *field != "psk").collect::<Vec<&str>>().join(" ");
        name = name.trim().replace("\u{1b}[1;90m> \u{1b}[0m", "").to_string();
        if !name.is_empty() {
            let asterisk_only = line.split_whitespace().last().unwrap_or("").to_string();
            let signal = asterisk_only.chars().filter(|c| *c == '*').collect::<String>();
            let network_row = NetworkRow { name, signal };
            network_rows.push(network_row);
        }
    }
    network_rows
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let device = env::args().skip(1).next().unwrap_or("wlan0".to_string());
    let output = Command::new("iwctl")
        .args(&["station", &device, "get-networks"])
        .output()?;
    let stdout = String::from_utf8_lossy(&output.stdout);

    let network_rows = parse_stdout(&stdout);

    for network_row in &network_rows {
        println!("Name: {} Signal: {}", network_row.name, network_row.signal);
    }

    Ok(())
}



