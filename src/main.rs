mod memory_info;

use std::process::Command;

fn main() {
    println!("Kernel Version Information");
    println!("=========================\n");

    // Get kernel version using uname -r
    match Command::new("uname").arg("-r").output() {
        Ok(output) => {
            let version = String::from_utf8_lossy(&output.stdout);
            println!("Kernel Release: {}", version.trim());
        }
        Err(e) => {
            eprintln!("Error getting kernel version: {}", e);
        }
    }

    // Get full kernel information using uname -a
    match Command::new("uname").arg("-a").output() {
        Ok(output) => {
            let info = String::from_utf8_lossy(&output.stdout);
            println!("Full Info: {}", info.trim());
        }
        Err(e) => {
            eprintln!("Error getting full kernel info: {}", e);
        }
    }

    // Show memory and disk information
    memory_info::show_memory_info();
    memory_info::show_disk_info();
}
