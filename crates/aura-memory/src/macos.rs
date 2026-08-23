use aura_core::errors::Result;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use tracing::info;
#[cfg(target_os = "macos")]
use tracing::warn;

pub fn spawn_macos_rss_monitor(pid: u32, budget_bytes: u64) -> Result<()> {
    info!(
        "Spawning macOS RSS Monitor thread (50ms interval) for PID={} with soft ceiling={} bytes",
        pid, budget_bytes
    );

    let running = Arc::new(AtomicBool::new(true));
    let running_clone = running.clone();

    thread::spawn(move || {
        // soft_limit is only used inside cfg(target_os = "macos"); on other platforms
        // the variable is intentionally suppressed.
        let soft_limit = (budget_bytes as f64 * 0.92) as u64;
        while running_clone.load(Ordering::Relaxed) {
            thread::sleep(Duration::from_millis(50));
            // Working-set RSS telemetry check using mach2::traps::mach_task_self (replaces deprecated libc::mach_task_self)
            #[cfg(target_os = "macos")]
            {
                use mach2::traps::mach_task_self;

                let mut task_info_data: libc::mach_task_basic_info = unsafe { std::mem::zeroed() };
                let mut count = (std::mem::size_of::<libc::mach_task_basic_info>()
                    / std::mem::size_of::<libc::integer_t>())
                    as libc::mach_msg_type_number_t;
                let kr = unsafe {
                    libc::task_info(
                        mach_task_self(),
                        libc::MACH_TASK_BASIC_INFO,
                        &mut task_info_data as *mut _ as *mut _,
                        &mut count,
                    )
                };
                if kr == libc::KERN_SUCCESS {
                    let rss = task_info_data.resident_size as u64;
                    if rss > soft_limit {
                        warn!(
                            "macOS RSS warning: process RSS {} bytes exceeds soft limit {} bytes",
                            rss, soft_limit
                        );
                    }
                }
            }
            // Suppress unused-variable warning on non-macOS platforms
            #[cfg(not(target_os = "macos"))]
            let _ = soft_limit;
        }
    });

    Ok(())
}
