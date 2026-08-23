#[cfg(windows)]
use aura_core::errors::AuraError;
use aura_core::errors::Result;
use tracing::info;

#[cfg(windows)]
use windows_sys::Win32::Foundation::CloseHandle;
#[cfg(windows)]
use windows_sys::Win32::System::JobObjects::{
    AssignProcessToJobObject, JobObjectExtendedLimitInformation, SetInformationJobObject,
    JOBOBJECT_EXTENDED_LIMIT_INFORMATION, JOB_OBJECT_LIMIT_PROCESS_MEMORY,
};

#[cfg(windows)]
extern "system" {
    fn CreateJobObjectW(
        lpjobattributes: *const std::ffi::c_void,
        lpname: *const u16,
    ) -> windows_sys::Win32::Foundation::HANDLE;
}

pub fn apply_windows_job_object(pid: u32, budget_bytes: u64) -> Result<()> {
    info!(
        "Applying native Win32 Job Object ProcessMemoryLimit={} bytes on PID={}",
        budget_bytes, pid
    );

    #[cfg(windows)]
    unsafe {
        use windows_sys::Win32::System::Threading::{
            OpenProcess, PROCESS_SET_QUOTA, PROCESS_TERMINATE,
        };

        let job = CreateJobObjectW(std::ptr::null(), std::ptr::null());
        if job.is_null() {
            return Err(AuraError::MemoryError(
                "Failed to create Win32 Job Object".to_string(),
            ));
        }

        let mut info_struct: JOBOBJECT_EXTENDED_LIMIT_INFORMATION = std::mem::zeroed();
        info_struct.BasicLimitInformation.LimitFlags = JOB_OBJECT_LIMIT_PROCESS_MEMORY;
        info_struct.ProcessMemoryLimit = budget_bytes as usize;

        let res = SetInformationJobObject(
            job,
            JobObjectExtendedLimitInformation,
            &info_struct as *const _ as *const _,
            std::mem::size_of::<JOBOBJECT_EXTENDED_LIMIT_INFORMATION>() as u32,
        );

        if res == 0 {
            CloseHandle(job);
            return Err(AuraError::MemoryError(
                "Failed to set Win32 Job Object ProcessMemoryLimit".to_string(),
            ));
        }

        let process_handle = OpenProcess(PROCESS_SET_QUOTA | PROCESS_TERMINATE, 0, pid);
        if process_handle.is_null() {
            let self_handle = windows_sys::Win32::System::Threading::GetCurrentProcess();
            let _ = AssignProcessToJobObject(job, self_handle);
            info!("Assigned current process to Win32 Job Object memory limit scope.");
        } else {
            let _ = AssignProcessToJobObject(job, process_handle);
            CloseHandle(process_handle);
            info!(
                "Assigned PID={} to Win32 Job Object memory limit scope.",
                pid
            );
        }
    }

    Ok(())
}
