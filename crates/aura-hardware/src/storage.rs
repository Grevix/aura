use aura_core::types::StorageProfile;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::time::Instant;

pub fn benchmark_storage(test_dir: &Path) -> StorageProfile {
    let test_file = test_dir.join(".aura_storage_test.tmp");

    // Default fallback values if direct IO test is restricted
    let mut seq_read_mbps = 1500.0;
    let mut random_iops = 100000;
    let storage_type = if cfg!(target_os = "windows") {
        "NVMe / SSD (Windows)".to_string()
    } else {
        "NVMe / SSD (Unix)".to_string()
    };

    // Fast non-destructive 16MB file read benchmark
    let file_size = 16 * 1024 * 1024;
    let buffer_size = 64 * 1024;

    if std::fs::write(&test_file, vec![0u8; file_size]).is_ok() {
        if let Ok(mut file) = File::open(&test_file) {
            let mut buf = vec![0u8; buffer_size];
            let start = Instant::now();
            let mut bytes_read = 0;
            while let Ok(n) = file.read(&mut buf) {
                if n == 0 {
                    break;
                }
                bytes_read += n;
            }
            let duration = start.elapsed().as_secs_f64();
            if duration > 0.0 {
                let mb_read = bytes_read as f64 / (1024.0 * 1024.0);
                seq_read_mbps = mb_read / duration;
                random_iops = (seq_read_mbps * 50.0) as u64;
            }
        }
        let _ = std::fs::remove_file(&test_file);
    }

    StorageProfile {
        storage_type,
        seq_read_mbps,
        random_iops,
    }
}
