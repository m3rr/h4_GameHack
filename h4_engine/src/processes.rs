use windows::Win32::System::Diagnostics::ToolHelp::{
    CreateToolhelp32Snapshot, Process32First, Process32Next, PROCESSENTRY32, TH32CS_SNAPPROCESS,
};
use windows::Win32::Foundation::CloseHandle;
use h4_shared::ProcessEntry;
use log::{error, info};

/// ProcessScanner handles locating and listing active game processes.
/// Comprehensive process scanning and categorization logic.
pub struct ProcessScanner;

impl ProcessScanner {
    /// Returns a list of all currently running processes.
    /// Returns a list of all currently running processes.
    pub fn list_processes() -> Vec<ProcessEntry> {
        info!("H4_Engine: Commencing system-wide process scan...");
        let mut processes = Vec::new();

        unsafe {
            let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);
            if let Ok(handle) = snapshot {
                if handle.is_invalid() {
                    error!("H4_Engine: Failed to create system snapshot.");
                    return processes;
                }

                let mut entry = PROCESSENTRY32 {
                    dwSize: std::mem::size_of::<PROCESSENTRY32>() as u32,
                    ..Default::default()
                };

                if Process32First(handle, &mut entry).is_ok() {
                    loop {
                        let mut name = entry.szExeFile.iter()
                            .take_while(|&&c| c != 0)
                            .map(|&c| c as u8 as char)
                            .collect::<String>();

                        // Handle special system names for clarity
                        if entry.th32ProcessID == 0 {
                            name = "[System Idle Process]".to_string();
                        } else if entry.th32ProcessID == 4 {
                            name = "System".to_string();
                        } else if name.is_empty() {
                            name = format!("Unknown (PID {})", entry.th32ProcessID);
                        }

                        let lower_name = name.to_lowercase();
                        let mut category = "Third Party".to_string();
                        if entry.th32ProcessID <= 100 || lower_name == "system" || lower_name == "registry" || lower_name == "[system idle process]" {
                            category = "System".to_string();
                        } else if lower_name == "smss.exe" || lower_name == "csrss.exe" || lower_name == "wininit.exe" 
                            || lower_name == "services.exe" || lower_name == "lsass.exe" || lower_name == "svchost.exe" 
                            || lower_name == "winlogon.exe" || lower_name.contains("windows") {
                            category = "Windows".to_string();
                        } else if lower_name.contains("steam") || lower_name.contains("epicgames") || lower_name.contains("galaxy") 
                            || lower_name.contains("game") || lower_name.contains("battle.net") || lower_name.contains("origin.exe") {
                            category = "Games".to_string();
                        } else if lower_name.contains("driver") || lower_name.contains("nvidia") || lower_name.contains("amd") 
                            || lower_name.contains("antivirus") || lower_name.contains("mbam") || lower_name.contains("defender") {
                            category = "Necessary".to_string();
                        }

                        let is_system = category == "System" || category == "Windows";

                        let status = if entry.cntThreads > 20 {
                            "Active".to_string()
                        } else if entry.cntThreads > 0 {
                            "Sleeping".to_string()
                        } else {
                            "Inactive".to_string()
                        };

                        processes.push(ProcessEntry {
                            pid: entry.th32ProcessID,
                            name,
                            status,
                            is_system,
                            category,
                        });

                        if Process32Next(handle, &mut entry).is_err() {
                            break;
                        }
                    }
                }
                let _ = CloseHandle(handle);
            }
        }

        info!("H4_Engine: Scan complete. Identified {} processes.", processes.len());
        processes
    }
}
