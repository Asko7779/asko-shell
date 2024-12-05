use std::io::{self, Write};
use std::process::exit;
use reqwest::blocking::get;
use get_if_addrs::get_if_addrs;

pub fn mainShell() -> Result<(), String> {
    fn localIPv4() -> Option<String> {
        let interfaces = get_if_addrs().unwrap();
        for iface in interfaces {
            if iface.is_loopback() {
                continue;
            }
            let ip = iface.ip();
            if ip.is_ipv4() {
                return Some(ip.to_string());
            }
        }
        None
    }
    fn publicIPv4() -> Result<String, reqwest::Error> {
        let response = get("https://api.ipify.org")?;
        let public_ip: String = response.text()?;
        Ok(public_ip)
    }
    println!("[ AskoShell V2.8 ]");
    println!("[ Dev: @Asko7779 ]\n");
    println!("Type 'help' to display commands.\n");

    let mut commandInput = String::new();
    loop {
        print!("as@localhost: ");
        io::stdout().flush().map_err(|e| format!("Error: {}", e))?;
        io::stdin()
            .read_line(&mut commandInput)
            .map_err(|e| format!("[!] Failed to read line: {}", e))?;
        let commandInput = commandInput.trim();

        if commandInput.is_empty() {
            println!("[!] No input provided");
        } else if commandInput == "exit" {
            exit(1);
        } else if commandInput == "ip" {
            match publicIPv4() {
                Ok(public_ip) => println!("[+] Public IP: {}", public_ip),
                Err(_) => println!("Error fetching IP."),
            }
            if let Some(local_ip) = localIPv4() {
                println!("[+] Local IPv4: {}", local_ip);
            } else {
                println!("No local IPv4 address found.");
            }
        } else if commandInput == "help" {
            println!("[   Available Commands  ]");
            println!("[ exit - Close AS       ]");
            println!("[ ip - Display IP info  ]");
            println!("[   MORE COMING SOON    ]");
        } else {
            println!("[+] Unrecognised Command");
        }
    }
}