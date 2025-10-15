use kernel_info::memory_info;
use std::process::Command;

#[test]
fn test_memory_info_runs_without_panic() {
    // Test that show_memory_info doesn't panic
    // This is an integration test - testing the whole function
    memory_info::show_memory_info();
}

#[test]
fn test_disk_info_runs_without_panic() {
    // Test that show_disk_info doesn't panic
    memory_info::show_disk_info();
}

#[test]
fn test_system_commands_are_available() {
    // Verify that required system commands exist
    let commands = vec!["uname", "vm_stat", "df", "sysctl"];
    
    for cmd in commands {
        let result = Command::new("which").arg(cmd).output();
        assert!(result.is_ok(), "Command '{}' should be available", cmd);
        
        if let Ok(output) = result {
            assert!(output.status.success(), "Command '{}' not found in PATH", cmd);
        }
    }
}

#[test]
fn test_uname_returns_kernel_version() {
    // Test that we can get kernel version
    let output = Command::new("uname")
        .arg("-r")
        .output()
        .expect("Failed to execute uname");
    
    assert!(output.status.success());
    let version = String::from_utf8_lossy(&output.stdout);
    assert!(!version.trim().is_empty(), "Kernel version should not be empty");
}

#[test]
fn test_sysctl_returns_memory_size() {
    // Test that we can get total memory
    let output = Command::new("sysctl")
        .arg("-n")
        .arg("hw.memsize")
        .output()
        .expect("Failed to execute sysctl");
    
    assert!(output.status.success());
    let mem_bytes = String::from_utf8_lossy(&output.stdout)
        .trim()
        .parse::<u64>();
    
    assert!(mem_bytes.is_ok(), "Should parse memory size as u64");
    assert!(mem_bytes.unwrap() > 0, "Memory size should be greater than 0");
}

#[test]
fn test_vm_stat_output_format() {
    // Test that vm_stat returns expected format
    let output = Command::new("vm_stat")
        .output()
        .expect("Failed to execute vm_stat");
    
    assert!(output.status.success());
    let info = String::from_utf8_lossy(&output.stdout);
    
    // Check that output contains expected fields
    assert!(info.contains("Pages active") || info.contains("pages active"));
    assert!(info.contains("Pages wired") || info.contains("pages wired"));
}

#[test]
fn test_df_output_format() {
    // Test that df returns expected format
    let output = Command::new("df")
        .arg("-h")
        .arg("/")
        .output()
        .expect("Failed to execute df");
    
    assert!(output.status.success());
    let info = String::from_utf8_lossy(&output.stdout);
    
    // Should have at least 2 lines (header + data)
    assert!(info.lines().count() >= 2, "df output should have at least 2 lines");
}
