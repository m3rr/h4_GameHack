use log::info;
use crate::MemoryManager;
use h4_shared::ValueType;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

/// ScanOperation defines how the current memory values are compared to previous findings.
#[derive(Clone, Debug, PartialEq)]
pub enum ScanOperation {
    ExactValue,
    Unchanged,
    Changed,
    Increased,
    Decreased,
    BiggerThan,
    SmallerThan,
}

/// DifferentialScanSession maintains the state of an iterative multi-pass search.
pub struct DifferentialScanSession {
    pub candidates: Vec<usize>,
    pub previous_values: Vec<u8>, // Flattened buffer of values corresponding to candidates
    pub value_type: ValueType,
}

impl DifferentialScanSession {
    pub fn new(initial_results: Vec<usize>, manager: &MemoryManager, vtype: ValueType) -> Self {
        let mut session = Self {
            candidates: initial_results,
            previous_values: Vec::new(),
            value_type: vtype,
        };
        session.snapshot_values(manager);
        session
    }

    /// Snapshots the current memory values for all candidate addresses.
    pub fn snapshot_values(&mut self, manager: &MemoryManager) {
        let size = self.value_type.size();
        let mut new_values = Vec::with_capacity(self.candidates.len() * size);
        
        for &addr in &self.candidates {
            let mut buf = vec![0u8; size];
            let mut read = 0;
            unsafe {
                use windows::Win32::System::Diagnostics::Debug::ReadProcessMemory;
                let _ = ReadProcessMemory(
                    manager.process_handle,
                    addr as *const _,
                    buf.as_mut_ptr() as *mut _,
                    size,
                    Some(&mut read),
                );
            }
            new_values.extend_from_slice(&buf);
        }
        self.previous_values = new_values;
    }

    /// Filters the candidates based on a new scan operation.
    pub fn filter(&mut self, manager: &MemoryManager, op: ScanOperation, target_val: Option<Vec<u8>>, cancel_token: Arc<AtomicBool>) {
        info!("SYSTEM >> Differential Filter Initiated: {:?}", op);
        let size = self.value_type.size();
        let mut filtered_candidates = Vec::new();
        let mut filtered_values = Vec::new();

        for (idx, &addr) in self.candidates.iter().enumerate() {
            if cancel_token.load(Ordering::SeqCst) { break; }
            
            let prev_start = idx * size;
            let prev_bytes = &self.previous_values[prev_start..prev_start + size];
            
            let mut curr_bytes = vec![0u8; size];
            let mut read = 0;
            unsafe {
                use windows::Win32::System::Diagnostics::Debug::ReadProcessMemory;
                let _ = ReadProcessMemory(
                    manager.process_handle,
                    addr as *const _,
                    curr_bytes.as_mut_ptr() as *mut _,
                    size,
                    Some(&mut read),
                );
            }
            if read != size { continue; }

            let matched = match op {
                ScanOperation::ExactValue => {
                    if let Some(ref target) = target_val {
                        curr_bytes == *target
                    } else { false }
                },
                ScanOperation::Unchanged => curr_bytes == prev_bytes,
                ScanOperation::Changed => curr_bytes != prev_bytes,
                ScanOperation::Increased => self.compare_numerical(prev_bytes, &curr_bytes, true),
                ScanOperation::Decreased => self.compare_numerical(prev_bytes, &curr_bytes, false),
                _ => false,
            };

            if matched {
                filtered_candidates.push(addr);
                filtered_values.extend_from_slice(&curr_bytes);
            }
        }

        info!("SYSTEM >> Filter Complete: {} candidates remaining.", filtered_candidates.len());
        self.candidates = filtered_candidates;
        self.previous_values = filtered_values;
    }

    fn compare_numerical(&self, prev: &[u8], curr: &[u8], increased: bool) -> bool {
        match self.value_type {
            ValueType::Int32 => {
                let p = i32::from_le_bytes(prev.try_into().unwrap_or([0;4]));
                let c = i32::from_le_bytes(curr.try_into().unwrap_or([0;4]));
                if increased { c > p } else { c < p }
            },
            ValueType::Float32 => {
                let p = f32::from_le_bytes(prev.try_into().unwrap_or([0;4]));
                let c = f32::from_le_bytes(curr.try_into().unwrap_or([0;4]));
                if increased { c > p } else { c < p }
            },
            ValueType::Float64 => {
                let p = f64::from_le_bytes(prev.try_into().unwrap_or([0;8]));
                let c = f64::from_le_bytes(curr.try_into().unwrap_or([0;8]));
                if increased { c > p } else { c < p }
            },
            _ => false,
        }
    }
}
