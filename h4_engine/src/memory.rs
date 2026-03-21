use std::ffi::c_void;
use windows::Win32::Foundation::{CloseHandle, HANDLE, INVALID_HANDLE_VALUE, WIN32_ERROR};
use windows::Win32::System::Diagnostics::Debug::{ReadProcessMemory, WriteProcessMemory};
use windows::Win32::System::Threading::{OpenProcess, PROCESS_ALL_ACCESS};
use thiserror::Error;
use log::{debug, error, info};

#[derive(Error, Debug)]
pub enum MemoryError {
    #[error("Failed to open process: {0}")]
    OpenProcessFailed(u32),
    #[error("Failed to read memory at {0:X}: {1}")]
    ReadFailed(usize, u32),
    #[error("Failed to write memory at {0:X}: {1}")]
    WriteFailed(usize, u32),
    #[error("Invalid process handle")]
    InvalidHandle,
}

/// A high-performance wrapper for Win32 memory operations.
/// Follows Rule 4 (Explicit Error Handling) and Rule 11 (Mandatory Logging).
pub struct MemoryManager {
    pub process_handle: HANDLE,
    pub pid: u32,
}

unsafe impl Send for MemoryManager {}
unsafe impl Sync for MemoryManager {}

impl MemoryManager {
    /// Attaches to a running process by its PID.
    /// Landmark: h4_engine/memory.rs - MemoryManager::attach
    pub fn attach(pid: u32) -> Result<Self, MemoryError> {
        info!("Attempting to attach to process with PID: {}", pid);
        
        unsafe {
            let handle = OpenProcess(PROCESS_ALL_ACCESS, false, pid)
                .map_err(|e| {
                    error!("Failed to open process {}: {:?}", pid, e);
                    MemoryError::OpenProcessFailed(WIN32_ERROR::from_error(&e).unwrap_or(WIN32_ERROR(0)).0)
                })?;

            if handle == INVALID_HANDLE_VALUE || handle.is_invalid() {
                error!("Invalid handle returned for PID {}", pid);
                return Err(MemoryError::InvalidHandle);
            }

            info!("Successfully attached to PID {}", pid);
            Ok(Self {
                process_handle: handle,
                pid,
            })
        }
    }

    pub fn get_process_name(&self) -> Result<String, MemoryError> {
        use windows::Win32::System::ProcessStatus::GetModuleBaseNameW;
        let mut buffer = [0u16; 260];
        unsafe {
            let len = GetModuleBaseNameW(self.process_handle, None, &mut buffer);
            if len == 0 {
                return Ok("Unknown".to_string());
            }
            Ok(String::from_utf16_lossy(&buffer[..len as usize]))
        }
    }

    /// Reads a value of type T from the process memory at the given address.
    /// Landmark: h4_engine/memory.rs - MemoryManager::read
    pub fn read<T: Copy>(&self, address: usize) -> Result<T, MemoryError> {
        debug!("Reading {} bytes from address {:X}", std::mem::size_of::<T>(), address);
        
        let mut buffer: T = unsafe { std::mem::zeroed() };
        let mut bytes_read: usize = 0;

        unsafe {
            let _ = ReadProcessMemory(
                self.process_handle,
                address as *const c_void,
                &mut buffer as *mut T as *mut c_void,
                std::mem::size_of::<T>(),
                Some(&mut bytes_read),
            ).map_err(|e| {
                let err = WIN32_ERROR::from_error(&e).unwrap_or(WIN32_ERROR(0));
                error!("Read failed at {:X}: {:?}", address, err);
                MemoryError::ReadFailed(address, err.0)
            })?;
        }

        if bytes_read != std::mem::size_of::<T>() {
            error!("Partial read at {:X}: expected {}, got {}", address, std::mem::size_of::<T>(), bytes_read);
            return Err(MemoryError::ReadFailed(address, 0));
        }

        Ok(buffer)
    }

    /// Writes a value of type T to the process memory at the given address.
    /// Landmark: h4_engine/memory.rs - MemoryManager::write
    pub fn write<T: Copy>(&self, address: usize, value: T) -> Result<(), MemoryError> {
        info!("Writing {} bytes to address {:X}", std::mem::size_of::<T>(), address);
        
        let mut bytes_written: usize = 0;

        unsafe {
            let _ = WriteProcessMemory(
                self.process_handle,
                address as *const c_void,
                &value as *const T as *const c_void,
                std::mem::size_of::<T>(),
                Some(&mut bytes_written),
            ).map_err(|e| {
                let err = WIN32_ERROR::from_error(&e).unwrap_or(WIN32_ERROR(0));
                error!("Write failed at {:X}: {:?}", address, err);
                MemoryError::WriteFailed(address, err.0)
            })?;
        }

        if bytes_written != std::mem::size_of::<T>() {
            error!("Partial write at {:X}: expected {}, got {}", address, std::mem::size_of::<T>(), bytes_written);
            return Err(MemoryError::WriteFailed(address, 0));
        }

        info!("Successfully wrote value to {:X}", address);
        Ok(())
    }
}

impl Drop for MemoryManager {
    fn drop(&mut self) {
        unsafe {
            if !self.process_handle.is_invalid() {
                debug!("Closing process handle for PID {}", self.pid);
                let _ = CloseHandle(self.process_handle);
            }
        }
    }
}
