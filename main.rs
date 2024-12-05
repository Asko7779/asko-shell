mod shell;
use shell::mainShell;
use std::process::exit;

fn main() {
    match mainShell() {
        Ok(_) => {
            println!("[+] Shell executed successfully.");
        }
        Err(e) => {
            eprintln!("[!] Error: {}", e);
            exit(1);
        }
    }
}