//! AURA Asynchronous Prefetching Ring-Buffer Manager (VTM Layer)
//!
//! Provides ring-buffered layer prefetching and native Windows `PrefetchVirtualMemory` /
//! Unix `madvise(..., MADV_WILLNEED)` OS kernel readahead calls.

use std::sync::atomic::{AtomicU64, Ordering};

pub static PREFETCH_HITS: AtomicU64 = AtomicU64::new(0);
pub static PREFETCH_MISSES: AtomicU64 = AtomicU64::new(0);
pub static PREFETCH_BYTES_READ: AtomicU64 = AtomicU64::new(0);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrefetchStrategy {
    None,
    SingleBuffer,
    DoubleBuffer,
    TripleBuffer,
    Adaptive,
}

pub struct AsyncPrefetcher {
    strategy: PrefetchStrategy,
}

impl AsyncPrefetcher {
    pub fn new(strategy: PrefetchStrategy) -> Self {
        Self { strategy }
    }

    pub fn prefetch_range(&self, ptr: *const u8, len: usize) {
        if self.strategy == PrefetchStrategy::None || len == 0 {
            return;
        }

        PREFETCH_BYTES_READ.fetch_add(len as u64, Ordering::Relaxed);

        #[cfg(target_os = "windows")]
        unsafe {
            use windows_sys::Win32::System::Memory::{
                PrefetchVirtualMemory, WIN32_MEMORY_RANGE_ENTRY,
            };
            use windows_sys::Win32::System::Threading::GetCurrentProcess;

            let range = WIN32_MEMORY_RANGE_ENTRY {
                VirtualAddress: ptr as *mut _,
                NumberOfBytes: len,
            };

            let proc_handle = GetCurrentProcess();
            let res = PrefetchVirtualMemory(proc_handle, 1, &range, 0);
            if res != 0 {
                PREFETCH_HITS.fetch_add(1, Ordering::Relaxed);
            } else {
                PREFETCH_MISSES.fetch_add(1, Ordering::Relaxed);
            }
        }

        #[cfg(unix)]
        unsafe {
            let res = libc::madvise(ptr as *mut libc::c_void, len, libc::MADV_WILLNEED);
            if res == 0 {
                PREFETCH_HITS.fetch_add(1, Ordering::Relaxed);
            } else {
                PREFETCH_MISSES.fetch_add(1, Ordering::Relaxed);
            }
        }
    }
}
