use std::process::Command;

pub fn show_memory_info() {
    println!("\nMemory Usage");
    println!("============");
    
    // Get total memory using sysctl
    let total_memory_gb = match Command::new("sysctl").arg("-n").arg("hw.memsize").output() {
        Ok(output) => {
            let mem_bytes = String::from_utf8_lossy(&output.stdout)
                .trim()
                .parse::<u64>()
                .unwrap_or(0);
            mem_bytes as f64 / 1024.0 / 1024.0 / 1024.0
        }
        Err(_) => 0.0,
    };
    
    // Get memory info using vm_stat on macOS
    match Command::new("vm_stat").output() {
        Ok(output) => {
            let info = String::from_utf8_lossy(&output.stdout);
            parse_memory_info(&info, total_memory_gb);
        }
        Err(e) => {
            eprintln!("Error getting memory info: {}", e);
        }
    }
}

fn parse_memory_info(vm_stat_output: &str, total_memory_gb: f64) {
    // Parse page size
    let page_size = 4096; // Default page size on macOS (4KB)
    
    let mut active_pages = 0;
    let mut wired_pages = 0;
    let mut compressed_pages = 0;
    
    for line in vm_stat_output.lines() {
        if line.contains("Pages active") {
            active_pages = extract_number(line);
        } else if line.contains("Pages wired down") {
            wired_pages = extract_number(line);
        } else if line.contains("Pages occupied by compressor") {
            compressed_pages = extract_number(line);
        }
    }
    
    // Convert to GB
    let active_gb = (active_pages * page_size) as f64 / 1024.0 / 1024.0 / 1024.0;
    let wired_gb = (wired_pages * page_size) as f64 / 1024.0 / 1024.0 / 1024.0;
    let compressed_gb = (compressed_pages * page_size) as f64 / 1024.0 / 1024.0 / 1024.0;
    let used_gb = active_gb + wired_gb + compressed_gb;
    
    let percentage = if total_memory_gb > 0.0 {
        (used_gb / total_memory_gb * 100.0) as u32
    } else {
        0
    };
    
    println!("Memory: {:.2} GiB / {:.2} GiB ({}%)", used_gb, total_memory_gb, percentage);
}

fn extract_number(line: &str) -> u64 {
    line.split_whitespace()
        .filter_map(|s| s.trim_end_matches('.').parse::<u64>().ok())
        .next()
        .unwrap_or(0)
}

pub fn show_disk_info() {
    println!("\nDisk Usage");
    println!("==========");
    
    // Get disk info using df command
    match Command::new("df").arg("-h").arg("/").output() {
        Ok(output) => {
            let info = String::from_utf8_lossy(&output.stdout);
            parse_disk_info(&info);
        }
        Err(e) => {
            eprintln!("Error getting disk info: {}", e);
        }
    }
}

fn parse_disk_info(df_output: &str) {
    // Skip header line and get the root filesystem info
    if let Some(line) = df_output.lines().nth(1) {
        let parts: Vec<&str> = line.split_whitespace().collect();
        
        if parts.len() >= 5 {
            let filesystem = parts[0];
            let size = parts[1];
            let used = parts[2];
            let available = parts[3];
            let capacity = parts[4];
            
            println!("Filesystem: {}", filesystem);
            println!("Size:       {}", size);
            println!("Used:       {}", used);
            println!("Available:  {}", available);
            println!("Capacity:   {}", capacity);
        }
    }
}

