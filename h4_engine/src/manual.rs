use log::{info, debug};
use crate::MemoryManager;
use windows::Win32::System::Memory::{VirtualQueryEx, MEMORY_BASIC_INFORMATION, MEM_COMMIT, PAGE_NOACCESS, PAGE_GUARD};
use std::ffi::c_void;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

/// ManualScanner handles user-initiated searches for specific values.
pub struct ManualScanner;

impl ManualScanner {
    /// Scans process memory for a specific value.
    /// MemoryManager search implementation.
    pub fn scan_for_value<T: Copy + PartialEq + Send + Sync>(
        manager: &MemoryManager,
        target_value: T,
        cancel_token: Arc<AtomicBool>,
    ) -> Vec<usize> {
        let mut results = Vec::new();
        let mut address: usize = 0;
        let mut mbi = MEMORY_BASIC_INFORMATION::default();
        let scan_start = std::time::Instant::now();
        let mut regions_scanned = 0;
        let mut total_bytes_scanned = 0;

        info!("H4_ENGINE: Starting Manual Value Scan.");
        
        // SELF-SCAN PROTECTION
        if manager.pid == std::process::id() {
            info!("H4_ENGINE: Self-scan detected and blocked. Safety first.");
            return results;
        }

        let value_size = std::mem::size_of::<T>();
        let target_bytes = unsafe {
            std::slice::from_raw_parts(&target_value as *const T as *const u8, value_size)
        };
        let first_byte = target_bytes[0];
        
        // REUSE BUFFER ( Optimized Performance )
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
            // Check for CANCEL ( Signal Responsiveness )
            if cancel_token.load(Ordering::Relaxed) {
                info!("H4_ENGINE: Scan cancelled by user.");
                return results;
            }

            if mbi.State == MEM_COMMIT 
               && (mbi.Protect & PAGE_NOACCESS).0 == 0 
               && (mbi.Protect & PAGE_GUARD).0 == 0 
            {
                regions_scanned += 1;
                let region_size = mbi.RegionSize;
                let region_start = mbi.BaseAddress as usize;

                if regions_scanned % 500 == 0 {
                    debug!("Progress: Scanned {} regions, found {} results...", regions_scanned, results.len());
                }

                let mut current_offset = 0;
                while current_offset < region_size {
                    if cancel_token.load(Ordering::Relaxed) { return results; }

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

                    if bytes_read >= value_size {
                        total_bytes_scanned += bytes_read;
                        let search_limit = bytes_read - value_size;
                        
                        // H4_FAST_PATH: Search for the first byte of target_value first!
                        // This skip 99% of bytes instantly.
                        let mut offset = 0;
                        while offset <= search_limit {
                            // Check for CANCEL more frequently during dense searches
                            if offset % 32768 == 0 && cancel_token.load(Ordering::Relaxed) {
                                return results;
                            }

                            if buffer[offset] == first_byte {
                                let potential_ptr = buffer[offset..].as_ptr() as *const T;
                                unsafe {
                                    if std::ptr::read_unaligned(potential_ptr) == target_value {
                                        results.push(region_start + current_offset + offset);
                                        if results.len() >= 1000 {
                                            info!("H4_ENGINE: Result cap reached (1000). Aborting scan.");
                                            return results;
                                        }
                                    }
                                }
                            }
                            offset += 1;
                        }
                    }

                    if bytes_read == 0 || current_offset + bytes_read >= region_size {
                        break;
                    }

                    current_offset += bytes_read - value_size + 1;
                }
            }
            address = mbi.BaseAddress as usize + mbi.RegionSize;
            if address == 0 { break; }
        }

        info!("H4_ENGINE: Manual scan complete in {:?}. Scanned {} regions ({} bytes). Found {} results.", 
            scan_start.elapsed(), regions_scanned, total_bytes_scanned, results.len());
        results
    }
}
