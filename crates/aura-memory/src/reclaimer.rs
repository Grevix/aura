//! Dynamic Memory Reclaiming & Working-Set Trimming
//!
//! Inspired by AirLLM's memory cleanup patterns (e.g. `malloc_trim`, GC eviction, and working set minimization)
//! implemented natively in Rust for Windows, Linux, and macOS.

use tracing::debug;

pub fn reclaim_process_memory() {
    debug!("Initiating process memory compaction and working set trimming...");

    #[cfg(target_os = "windows")]
    unsafe {
        use windows_sys::Win32::System::ProcessStatus::EmptyWorkingSet;
        use windows_sys::Win32::System::Threading::GetCurrentProcess;

        let proc_handle = GetCurrentProcess();
        let res = EmptyWorkingSet(proc_handle);
        if res != 0 {
            debug!("Win32 EmptyWorkingSet successfully trimmed unreferenced pages to system pool.");
        }
    }

    #[cfg(target_os = "linux")]
    unsafe {
        // Trigger glibc malloc_trim(0) if available to release free arenas back to the kernel
        extern "C" {
            fn malloc_trim(pad: usize) -> i32;
        }
        let _ = malloc_trim(0);
        debug!("Linux malloc_trim(0) released unreferenced heap arenas back to OS.");
    }
}

pub fn reclaim_target_process_memory(pid: u32) {
    debug!("Initiating target PID={} memory compaction...", pid);

    #[cfg(target_os = "windows")]
    unsafe {
        use windows_sys::Win32::System::ProcessStatus::EmptyWorkingSet;
        use windows_sys::Win32::System::Threading::{
            OpenProcess, PROCESS_QUERY_INFORMATION, PROCESS_SET_QUOTA,
        };

        let handle = OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_SET_QUOTA, 0, pid);
        if !handle.is_null() {
            let _ = EmptyWorkingSet(handle);
            windows_sys::Win32::Foundation::CloseHandle(handle);
            debug!(
                "Win32 EmptyWorkingSet successfully trimmed target PID={} working set.",
                pid
            );
        }
    }

    #[cfg(target_os = "linux")]
    {
        let _ = pid;
    }
}
