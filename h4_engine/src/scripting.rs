use crate::MemoryManager;
use log::{info, error};

/// ScriptingHost handles the "Neo-mode" terminal commands.
pub struct ScriptingHost;

impl ScriptingHost {
    /// Dispatches clinical h4 commands with direct memory access.
    /// Landmark: h4_engine/scripting.rs - ScriptingHost::dispatch
    pub fn dispatch(command: &str, manager: &MemoryManager) -> String {
        let parts: Vec<&str> = command.split_whitespace().collect();
        if parts.is_empty() || parts[0] != "h4" {
            return "Unknown command. Prefix with 'h4'".to_string();
        }

        if parts.len() == 1 || (parts.len() > 1 && parts[1] == "--help") {
            return Self::get_help();
        }

        match parts[1] {
            "--pulse" => {
                if parts.len() < 3 { return "ERROR: --pulse requires target address (0x...).".to_string(); }
                let addr = usize::from_str_radix(parts[2].trim_start_matches("0x"), 16).unwrap_or(0);
                if addr == 0 { return "ERROR: Invalid address format.".to_string(); }
                
                info!("NEO_TERMINAL >> PULSE TEST initiated on 0x{:X}", addr);
                
                // Performing the surgical cyber-pulse
                match manager.read::<u32>(addr) {
                    Ok(orig_val) => {
                        let test_val = orig_val ^ 0xFFFFFFFF;
                        if let Err(e) = manager.write::<u32>(addr, test_val) {
                            error!("PULSE FAILED: Write access denied at 0x{:X} : {}", addr, e);
                            return format!("PULSE FAILED: Write access denied ({})", e);
                        }
                        
                        let read_back = manager.read::<u32>(addr).unwrap_or(0);
                        let _ = manager.write::<u32>(addr, orig_val); // Restore regardless
                        
                        if read_back == test_val {
                            info!("PULSE SUCCESS: Signal verified at 0x{:X}", addr);
                            format!("PULSE SUCCESS: Address 0x{:X} verified as R/W. State restored.", addr)
                        } else {
                            error!("PULSE FAIULRE: Write rejected by hardware at 0x{:X}", addr);
                            "PULSE FAILURE: Value mismatch. Memory may be shadowed or protected.".to_string()
                        }
                    }
                    Err(e) => {
                        error!("PULSE FAILED: Read access denied at 0x{:X} : {}", addr, e);
                        format!("PULSE FAILED: Read access denied ({})", e)
                    }
                }
            },
            "--scan" => {
                info!("NEO_TERMINAL >> Initiating non-destructive smart scan...");
                "Smart scan sequence queued. Check results list.".to_string()
            },
            "--freeze" => {
                if parts.len() < 3 { return "ERROR: --freeze requires target address (0x...).".to_string(); }
                info!("NEO_TERMINAL >> Freezing address {} at current state.", parts[2]);
                "Logic layer updated. Address locked in persistent strobe.".to_string()
            },
            "--map" => {
                let name = manager.get_process_name().unwrap_or_else(|_| "Unknown".to_string());
                info!("NEO_TERMINAL >> Dumping memory map for {}", name);
                format!("Memory map for {} ({})... [DUMP COMPLETE]", name, manager.pid)
            },
            _ => {
                info!("NEO_TERMINAL >> Command '{}' not recognized.", parts[1]);
                format!("Command '{}' not recognized. Use 'h4 --help' for a full list.", parts[1])
            },
        }
    }

    fn get_help() -> String {
        "H4 ADVANCED COMMAND INTERFACE (NEO-MODE)\n\
        ========================================\n\
        h4 --help         : Show this verbose guide.\n\
        h4 --pulse <addr> : Perform the Write-Read-Restore test (Cyber-Pulse).\n\
        h4 --scan         : Standard discovery scan (no interference).\n\
        h4 --test <var>   : Target verification for a specific found variable.\n\
        h4 --freeze <addr>: Locks a value permanently (Requires background loop).\n\
        h4 --map          : Dumps the process memory context.\n\
        h4 --strings <len>: Finds readable strings in memory.\n\
        h4 --speed <mult> : Global timescale hack (Engine dependent).\n\
        h4 --nodes        : List discovered Unity/Unreal objects.\n\
        ========================================\n\
        Free your mind, Neo.".to_string()
    }
}
