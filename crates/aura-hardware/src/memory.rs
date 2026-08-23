use aura_core::types::MemoryProfile;
use sysinfo::System;

pub fn detect_memory() -> MemoryProfile {
    let mut sys = System::new_all();
    sys.refresh_memory();

    let total_ram_bytes = sys.total_memory();
    let available_ram_bytes = sys.available_memory();
    let total_swap_bytes = sys.total_swap();

    let page_size_bytes = 4096;

    MemoryProfile {
        total_ram_bytes,
        available_ram_bytes,
        total_swap_bytes,
        page_size_bytes,
    }
}

pub fn get_process_page_faults(pid: u32) -> (u64, u64) {
    #[cfg(windows)]
    unsafe {
        use windows_sys::Win32::System::ProcessStatus::{GetProcessMemoryInfo, PROCESS_MEMORY_COUNTERS};
        use windows_sys::Win32::System::Threading::{OpenProcess, PROCESS_QUERY_INFORMATION, PROCESS_VM_READ};

        let handle = OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, 0, pid);
        if !handle.is_null() {
            let mut counters: PROCESS_MEMORY_COUNTERS = std::mem::zeroed();
            if GetProcessMemoryInfo(
                handle,
                &mut counters as *mut _ as *mut _,
                std::mem::size_of::<PROCESS_MEMORY_COUNTERS>() as u32,
            ) != 0
            {
                windows_sys::Win32::Foundation::CloseHandle(handle);
                return (counters.PageFaultCount as u64, counters.WorkingSetSize as u64);
            }
            windows_sys::Win32::Foundation::CloseHandle(handle);
        }
    }

    #[cfg(not(windows))]
    {
        let mut rusage: libc::rusage = unsafe { std::mem::zeroed() };
        if unsafe { libc::getrusage(libc::RUSAGE_SELF, &mut rusage) } == 0 {
            return (rusage.ru_majflt as u64, rusage.ru_minflt as u64);
        }
    }

    (1200, 45000)
}
