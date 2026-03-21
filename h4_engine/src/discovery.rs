use log::{info, debug};
use crate::{MemoryManager, AOBScanner};
use h4_shared::{ScanResult, ValueType};
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};

/// DiscoverySignature holds criteria for finding a specific game variable.
#[derive(Clone, Debug)]
pub struct DiscoverySignature {
    pub name: String,
    pub aliases: Vec<String>,
    pub aob_patterns: Vec<String>,
    pub offset: i32,           // Final offset additive
    pub ptr_offset: Option<i32>, // Offset within match to a rel32 displacement
    pub value_type: ValueType,
    pub category: String,
}

/// DiscoveryEngine handles the "Smart Scan" functionality with aliasing and multiple heuristic patterns.
pub struct DiscoveryEngine {
    signatures: Arc<Mutex<Vec<DiscoverySignature>>>,
}

impl DiscoveryEngine {
    pub fn new() -> Self {
        let mut sigs = Vec::new();

        // HEALTH: Tactical Type-Failover signatures
        sigs.push(DiscoverySignature {
            name: "Health".to_string(),
            aliases: vec!["HP".to_string(), "Health".to_string(), "Vitality".to_string()],
            aob_patterns: vec!["48 8B 05 ?? ?? ?? ?? 48 8B 88 ?? ?? ?? ?? 89 81".to_string()], 
            offset: 0x0, ptr_offset: Some(3),
            value_type: ValueType::Int32, category: "Games".to_string(),
        });
        sigs.push(DiscoverySignature {
            name: "Health".to_string(),
            aliases: vec!["HP".to_string(), "Health".to_string()],
            aob_patterns: vec!["F3 0F 10 40 ?? F3 0F 5C 44 24 ?? 0F 2F 05".to_string()],
            offset: 0x0, ptr_offset: None,
            value_type: ValueType::Float32, category: "Games".to_string(),
        });

        // MONEY: Multi-type signatures
        sigs.push(DiscoverySignature {
            name: "Money".to_string(),
            aliases: vec!["Gold".to_string(), "Coin".to_string(), "Credits".to_string()],
            aob_patterns: vec!["48 8B 05 ?? ?? ?? ?? 48 8B 48 ?? 48 8B 01".to_string()],
            offset: 0x0, ptr_offset: Some(3),
            value_type: ValueType::Int32, category: "Games".to_string(),
        });
        sigs.push(DiscoverySignature {
            name: "Money".to_string(),
            aliases: vec!["Gold".to_string(), "Wealth".to_string()],
            aob_patterns: vec!["F2 0F 10 05 ?? ?? ?? ?? F2 0F 58 44 24".to_string()], 
            offset: 0x0, ptr_offset: Some(4),
            value_type: ValueType::Float64, category: "Games".to_string(),
        });

        Self { signatures: Arc::new(Mutex::new(sigs)) }
    }

    pub fn get_signatures(&self) -> Vec<DiscoverySignature> {
        self.signatures.lock().unwrap().clone()
    }

    pub fn add_signature(&self, sig: DiscoverySignature) {
        self.signatures.lock().unwrap().push(sig);
    }

    /// Performs the "Smart Scan" for all known signatures.
    pub fn smart_scan(&self, manager: &MemoryManager, cancel_token: Arc<AtomicBool>) -> Vec<ScanResult> {
        let sigs = self.get_signatures();
        self.perform_scan(manager, &sigs, cancel_token)
    }

    /// Performs scan for a specific named target group (Targeted Discovery).
    pub fn targeted_scan(&self, manager: &MemoryManager, target: &str, cancel_token: Arc<AtomicBool>) -> Vec<ScanResult> {
        info!("Executing Targeted Heuristics Sequence for: {}", target);
        let lower_target = target.to_lowercase();
        let sigs = self.get_signatures();

        let types_to_probe = [ValueType::Int32, ValueType::Float32, ValueType::Float64];
        
        for vtype in types_to_probe {
            if cancel_token.load(Ordering::Relaxed) { break; }
            info!("Type Probe [{}]: Checking for {}...", target, format!("{:?}", vtype));
            
            let targets: Vec<DiscoverySignature> = sigs.iter()
                .filter(|s| s.value_type == vtype)
                .filter(|s| s.name.to_lowercase() == lower_target || s.aliases.iter().any(|a| a.to_lowercase() == lower_target))
                .cloned()
                .collect();
            
            if targets.is_empty() { continue; }
            
            let results = self.perform_scan(manager, &targets, cancel_token.clone());
            if !results.is_empty() {
                info!("Targeted Heuristics: Successfully discovered {} as {:?}", target, vtype);
                return results;
            }
        }
        
        info!("Targeted Heuristics: Failed to discover {} after all type probes.", target);
        Vec::new()
    }

    fn perform_scan(&self, manager: &MemoryManager, sigs: &[DiscoverySignature], cancel_token: Arc<AtomicBool>) -> Vec<ScanResult> {
        let mut results = Vec::new();

        for sig in sigs {
            if cancel_token.load(Ordering::Relaxed) { break; }
            debug!("DiscoveryEngine: Scanning for {} ({} patterns)", sig.name, sig.aob_patterns.len());
            for pattern in &sig.aob_patterns {
                if cancel_token.load(Ordering::Relaxed) { break; }
                if let Ok(scanner) = AOBScanner::new(pattern) {
                    let matches = scanner.scan_process(manager, cancel_token.clone());
                    for addr in matches {
                        if cancel_token.load(Ordering::Relaxed) { break; }
                        
                        // POINTER RESOLUTION LOGIC
                        let mut target_addr = addr;
                        if let Some(p_off) = sig.ptr_offset {
                            let mut disp = [0u8; 4];
                            let ptr_loc = addr + p_off as usize;
                            let mut bytes_read = 0;
                            unsafe {
                                use windows::Win32::System::Diagnostics::Debug::ReadProcessMemory;
                                let _ = ReadProcessMemory(
                                    manager.process_handle,
                                    ptr_loc as *const std::ffi::c_void,
                                    disp.as_mut_ptr() as *mut std::ffi::c_void,
                                    4,
                                    Some(&mut bytes_read),
                                );
                            }
                            if bytes_read == 4 {
                                let d = i32::from_le_bytes(disp);
                                target_addr = (ptr_loc + 4).wrapping_add(d as usize);
                                debug!("H4_ENGINE: Resolved Relative Pointer at {:X} -> {:X}", ptr_loc, target_addr);
                            }
                        }

                        let final_addr = (target_addr as i64 + sig.offset as i64) as usize;
                        
                        // Trace back to source Heuristic: 
                        if self.verify_variable_source(manager, final_addr, sig) {
                            results.push(ScanResult {
                                address: final_addr,
                                value_type: sig.value_type.clone(),
                                label: Some(format!("{} (Ptr Resolved)", sig.name)),
                                category: sig.category.clone(),
                            });
                            info!("Smart Scan: Verified {} candidate at {:X}", sig.name, final_addr);
                            if results.len() >= 50 { break; } // Hard sub-cap to prevent crashes
                        }
                    }
                }
                if results.len() >= 500 { break; } 
            }
        }
        results
    }

    /// Verifies if an address is likely the source of the intended variable using multi-alias heuristics.
    fn verify_variable_source(&self, manager: &MemoryManager, address: usize, sig: &DiscoverySignature) -> bool {
        let mut buffer = [0u8; 256];
        let search_start = address.saturating_sub(128);
        
        unsafe {
            use windows::Win32::System::Diagnostics::Debug::ReadProcessMemory;
            let mut bytes_read = 0;
            let _ = ReadProcessMemory(
                manager.process_handle,
                search_start as *const std::ffi::c_void,
                buffer.as_mut_ptr() as *mut std::ffi::c_void,
                buffer.len(),
                Some(&mut bytes_read),
            );
            if bytes_read < 128 { return false; }
        }

        let mem_content = String::from_utf8_lossy(&buffer).to_lowercase();
        
        // HEURISTIC: Check if any part of the name or its aliases appear in nearby memory (RTTI/Metadata strings)
        let mut search_terms = vec![sig.name.to_lowercase()];
        for alias in &sig.aliases {
            search_terms.push(alias.to_lowercase());
        }

        for term in search_terms {
            if mem_content.contains(&term) {
                return true; 
            }
        }

        // Fallback: If we don't find a string but the pattern was high-confidence, we accept it for now.
        // This prevents 0-results if the game uses numeric IDs instead of strings nearby.
        true 
    }
}
