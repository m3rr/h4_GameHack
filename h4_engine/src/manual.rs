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

        info!("H4_ENGINE: Starting Manual Value Scan (Bitwise Strategy).");
        
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
                        info!("H4_ENGINE: Manual Scan self-protection enabled. Skipping region {:X}-{:X}", skip_range.0, skip_range.1);
                    }
                }
            }
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
            // Check for CANCEL ( NUCLEAR Responsiveness )
            if cancel_token.load(Ordering::SeqCst) {
                info!("SYSTEM >> Manual Scan Sequence: Abort confirmed via User Signal.");
                return results;
            }

            let region_start = mbi.BaseAddress as usize;
            let region_size = mbi.RegionSize;

            if mbi.State == MEM_COMMIT 
               && (mbi.Protect & PAGE_NOACCESS).0 == 0 
               && (mbi.Protect & PAGE_GUARD).0 == 0 
               && !(region_start >= skip_range.0 && region_start < skip_range.1)
            {
                regions_scanned += 1;
                if regions_scanned % 100 == 0 {
                    debug!("SYSTEM >> Excavating Region Segment #{}: {:X} (Size: {:X})", regions_scanned, region_start, region_size);
                }

                let mut current_offset = 0;
                while current_offset < region_size {
                    if cancel_token.load(Ordering::SeqCst) { 
                        info!("SYSTEM >> Manual Scan Sequence: Early exit during region excavation.");
                        return results; 
                    }

                    let to_read = std::cmp::min(CHUNK_SIZE, region_size - current_offset);
                    let mut bytes_read: usize = 0;

                    unsafe {
                        use windows::Win32::System::Diagnostics::Debug::ReadProcessMemory;
                        let res = ReadProcessMemory(
                            manager.process_handle,
                            (region_start + current_offset) as *const c_void,
                            buffer.as_mut_ptr() as *mut c_void,
                            to_read,
                            Some(&mut bytes_read),
                        );
                        if res.is_err() { break; }
                    }

                    if bytes_read >= value_size {
                        total_bytes_scanned += bytes_read;
                        let search_limit = bytes_read - value_size;
                        
                        let mut offset = 0;
                        while offset <= search_limit {
                            // Check for CANCEL every 64kb of buffer
                            if offset & 0xFFFF == 0 && cancel_token.load(Ordering::SeqCst) {
                                return results;
                            }

                            // H4_FAST_PATH: Absolute Bitwise-Byte-Slice Comparison
                            // This bypasses the FPU and avoids signaling-NaN traps or alignment stalls.
                            if buffer[offset] == first_byte {
                                if &buffer[offset..offset+value_size] == target_bytes {
                                    let found_addr = region_start + current_offset + offset;
                                    results.push(found_addr);
                                    
                                    if results.len() >= 1000 {
                                        info!("SYSTEM >> Discovery Threshold BREACHED (1000). Terminating excavation pass.");
                                        return results;
                                    }
                                }
                            }
                            offset += 1;
                        }
                        current_offset += bytes_read - value_size + 1;
                    } else { break; }

                    if current_offset >= region_size { break; }
                }
            }
            address = mbi.BaseAddress as usize + mbi.RegionSize;
            if address == 0 { break; }
        }

        info!("H4_ENGINE: Manual scan complete in {:?}. Scanned {} regions ({} bytes). Found {} results.", 
            scan_start.elapsed(), regions_scanned, total_bytes_scanned, results.len());
        results
    }

    /// Scans memory looking for pointers that point to a specific target address.
    /// This is crucial for tracking dynamic variables back to their static pointers.
    pub fn find_pointers(
        manager: &MemoryManager,
        target_address: usize,
        cancel_token: Arc<AtomicBool>,
    ) -> Vec<usize> {
        let mut results = Vec::new();
        let target_bytes = target_address.to_le_bytes(); // 64-bit pointer
        
        info!("SYSTEM >> Pointer Excavation Sequence Initiated. Target: {:X}", target_address);
        
        let mut address: usize = 0;
        let mut mbi = MEMORY_BASIC_INFORMATION::default();
        const CHUNK_SIZE: usize = 1024 * 1024 * 4;
        let mut buffer = vec![0u8; CHUNK_SIZE];

        while unsafe {
            VirtualQueryEx(
                manager.process_handle,
                Some(address as *const c_void),
                &mut mbi,
                std::mem::size_of::<MEMORY_BASIC_INFORMATION>(),
            )
        } != 0 {
            if cancel_token.load(Ordering::SeqCst) { break; }

            let region_start = mbi.BaseAddress as usize;
            let region_size = mbi.RegionSize;

            if mbi.State == MEM_COMMIT 
               && (mbi.Protect & PAGE_NOACCESS).0 == 0 
               && (mbi.Protect & PAGE_GUARD).0 == 0 
            {
                let mut current_offset = 0;
                while current_offset < region_size {
                    if cancel_token.load(Ordering::SeqCst) { break; }
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

                    if bytes_read >= 8 {
                        for i in 0..=(bytes_read - 8) {
                            if buffer[i..i+8] == target_bytes {
                                results.push(region_start + current_offset + i);
                            }
                        }
                    }
                    current_offset += to_read;
                }
            }
            address = region_start + region_size;
        }

        info!("SYSTEM >> Pointer Excavation Complete. Discovered {} potential paths.", results.len());
        results
    }
}
