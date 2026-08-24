# Memory Budget Enforcement: Job Object Process Trees vs Peak RSS Telemetry

> Technical note on Win32 Job Object limits, Linux cgroup scopes, and process tree memory measurement.

---

## 1. Win32 Job Object vs Global RSS Measurement

When executing `aura run --memory 4G`:

1. **Job Object Enforcement**: The child backend process (e.g. `llama-server.exe` PID=14152) is assigned directly to a Win32 Job Object with `JOB_OBJECT_LIMIT_PROCESS_MEMORY` set to $4,000,000,000\text{ bytes}$ ($4.00\text{ GB}$).
2. **Process Tree Scope**: The Win32 Job Object strictly caps the committed private bytes of the child inference process.
3. **Telemetry Measurement (RSS)**: Global Resident Set Size (RSS) telemetry reported in the run output includes:
   - Shared memory-mapped files (e.g., GGUF weight blobs read via OS file cache)
   - Parent CLI process memory
   - Child process private working set
4. **Conclusion**: While the child process committed memory is capped by the OS kernel to $4.00\text{ GB}$, shared read-only mapped pages across the process tree may reflect up to $\sim 4.92\text{ GB}$ total working set in global telemetry.

---

## 2. Linux cgroup v2 Alignment

On Linux hosts, AURA binds the child PID to `/sys/fs/cgroup/aura_<PID>/memory.max = 4G`. Unlike Win32 private commit limits, Linux cgroup v2 `memory.max` counts both anonymous private memory and page cache allocated by the cgroup, triggering kernel-level reclaim or OOM killing if exceeded.
