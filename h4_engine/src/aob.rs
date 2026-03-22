use log::{debug, info};
use crate::MemoryManager;
use windows::Win32::System::Memory::{VirtualQueryEx, MEMORY_BASIC_INFORMATION, MEM_COMMIT, PAGE_NOACCESS, PAGE_GUARD};
use std::ffi::c_void;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

/// Pattern element for AOB scanning.
#[derive(Debug, Clone, PartialEq)]
enum PatternByte {
    Byte(u8),
    Wildcard,
}

/// A signature-based scanner for finding addresses in memory.
/// Supports wildcards in the pattern (e.g. "48 8B 05 ?? ?? ?? ??").
pub struct AOBScanner {
    pattern: Vec<PatternByte>,
}

impl AOBScanner {
    /// Parses a string pattern like "48 8B 05 ?? ?? ?? ??" into an AOBScanner.
    pub fn new(pattern_str: &str) -> Result<Self, String> {
        let mut pattern = Vec::new();
        for token in pattern_str.split_whitespace() {
            if token == "??" || token == "?" {
                pattern.push(PatternByte::Wildcard);
            } else {
                let byte = u8::from_str_radix(token, 16)
                    .map_err(|e| format!("Invalid byte in pattern: {}. Error: {}", token, e))?;
                pattern.push(PatternByte::Byte(byte));
            }
        }
        if pattern.is_empty() { return Err("Pattern cannot be empty".to_string()); }
        Ok(Self { pattern })
    }

    /// Scans a buffer for the pattern. Returns the relative offset if found.
    pub fn find_in_buffer(&self, buffer: &[u8]) -> Option<usize> {
        if buffer.len() < self.pattern.len() { return None; }

        let first_byte = if let PatternByte::Byte(fb) = self.pattern[0] { Some(fb) } else { None };
        let mut i = 0;
        let limit = buffer.len() - self.pattern.len();
        
        while i <= limit {
            if let Some(fb) = first_byte {
                if buffer[i] != fb { i += 1; continue; }
            }

            let mut matched = true;
            for (j, pat_byte) in self.pattern.iter().enumerate().skip(1) {
                match pat_byte {
                    PatternByte::Wildcard => continue,
                    PatternByte::Byte(b) => {
                        if buffer[i + j] != *b { matched = false; break; }
                    }
                }
            }
            if matched { return Some(i); }
            i += 1;
        }
        None
    }

    /// Scans the entire process memory for this pattern.
    pub fn scan_process(&self, manager: &MemoryManager, cancel_token: Arc<AtomicBool>) -> Vec<usize> {
        let mut results = Vec::new();
        let mut address: usize = 0;
        let mut mbi = MEMORY_BASIC_INFORMATION::default();
        let scan_start = std::time::Instant::now();
        let mut regions_scanned = 0;
        let mut total_bytes_scanned = 0;

        info!("H4_ENGINE: Starting AOB scan. Pattern size: {}", self.pattern.len());

        // SELF-SCAN PROTECTION: Skip our own executable region to prevent feedback loops
        let self_pid = std::process::id();
        let mut skip_range = (0usize, 0usize);
        if manager.pid == self_pid {
            use windows::Win32::System::LibraryLoader::GetModuleHandleW;
            use windows::Win32::System::Memory::VirtualQuery;
            unsafe {
                if let Ok(module) = GetModuleHandleW(None) {
                    let mut mbi_self = MEMORY_BASIC_INFORMATION::default();
                    if VirtualQuery(Some(module.0 as *const c_void), &mut mbi_self, std::mem::size_of::<MEMORY_BASIC_INFORMATION>()) != 0 {
                        skip_range = (mbi_self.BaseAddress as usize, mbi_self.BaseAddress as usize + mbi_self.RegionSize);
                        info!("H4_ENGINE: Self-scan detected. Constraining heuristics to skip scanner core at {:X}-{:X}", skip_range.0, skip_range.1);
                    }
                }
            }
        }

        // Reuse buffer to avoid excessive allocations
        const CHUNK_SIZE: usize = 1024 * 1024 * 2;
        let mut buffer = vec![0u8; CHUNK_SIZE];

        while unsafe {
            VirtualQueryEx(
                manager.process_handle,
                Some(address as *const c_void),
                &mut mbi,
                std::mem::size_of::<MEMORY_BASIC_INFORMATION>(),
            )
        } != 0 {
            // Check for CANCEL (NUCLEAR Responsiveness)
            if cancel_token.load(Ordering::SeqCst) {
                info!("H4_ENGINE: AOB Scan aborted by User Signal.");
                return results;
            }

            let region_start = mbi.BaseAddress as usize;
            let region_size = mbi.RegionSize;

            // Only scan committed memory that isn't protected/guard paging
            // AND Skip our own core if we are scanning ourselves
            if mbi.State == MEM_COMMIT 
               && (mbi.Protect & PAGE_NOACCESS).0 == 0 
               && (mbi.Protect & PAGE_GUARD).0 == 0 
               && !(region_start >= skip_range.0 && region_start < skip_range.1)
            {
                regions_scanned += 1;
                let mut current_offset = 0;
                while current_offset < region_size {
                    if cancel_token.load(Ordering::SeqCst) { return results; }

                    let to_read = std::cmp::min(CHUNK_SIZE, region_size - current_offset);
                    let mut bytes_read: usize = 0;

                    unsafe {
                        use windows::Win32::System::Diagnostics::Debug::ReadProcessMemory;
                        let _ = ReadProcessMemory(
                            manager.process_handle,
                            (region_start + current_offset) as *const c_void,
                            buffer.as_mut_ptr() as *mut c_void,
                            to_read,
                            Some(&mut bytes_read),
                        );
                    }

                    if bytes_read >= self.pattern.len() {
                        total_bytes_scanned += bytes_read;
                        let search_buf = &buffer[..bytes_read];
                        let mut find_offset = 0;

                        while let Some(found_idx) = self.find_in_buffer(&search_buf[find_offset..]) {
                            // Check for CANCEL very frequently in inner loop
                            if find_offset % 32768 == 0 && cancel_token.load(Ordering::SeqCst) { 
                                info!("H4_ENGINE: AOB Scan aborted in inner heuristic loop.");
                                return results; 
                            }

                            let absolute_addr = region_start + current_offset + find_offset + found_idx;
                            results.push(absolute_addr);
                            
                            // UI CRASH PROTECTION: Cap results to prevent Slint from dying on huge result sets
                            if results.len() >= 1000 {
                                info!("H4_ENGINE: Result cap reached (1000). Aborting scan.");
                                return results;
                            }
                            
                            find_offset += found_idx + 1;
                            if find_offset >= search_buf.len() { break; }
                        }
                        
                        // Move to next chunk, but keep enough overlap for the pattern
                        current_offset += bytes_read - self.pattern.len() + 1;
                    } else {
                        // Region either closed or too small for pattern
                        break;
                    }

                    if current_offset >= region_size {
                        break;
                    }
                }
            }    
            if regions_scanned % 100 == 0 {
                debug!("Progress: Scanned {} regions, found {} matches so far...", regions_scanned, results.len());
            }

            address = mbi.BaseAddress as usize + mbi.RegionSize;
            // Prevent overflow loop
            if address == 0 { break; }
        }

        info!("H4_ENGINE: Scan complete in {:?}. Scanned {} regions ({} bytes). Found {} matches.", 
            scan_start.elapsed(), regions_scanned, total_bytes_scanned, results.len());
        results
    }
}
