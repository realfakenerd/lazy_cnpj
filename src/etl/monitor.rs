use std::{path::Path, thread};

use sysinfo::{Disk, Disks};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PerformanceProfile {
    Conservative,
    Moderate,
    HighPerformance,
}

impl PerformanceProfile {
    pub fn max_concurrent_downloads(self) -> usize {
        match self {
            PerformanceProfile::Conservative => 1,
            PerformanceProfile::Moderate => 2,
            PerformanceProfile::HighPerformance => 4,
        }
    }

    pub fn csv_parsing_workers(self) -> usize {
        match self {
            PerformanceProfile::Conservative => 1,
            PerformanceProfile::Moderate => 2,
            PerformanceProfile::HighPerformance => 4,
        }
    }

    pub fn db_batch_size(self) -> usize {
        match self {
            PerformanceProfile::Conservative => 25_000,
            PerformanceProfile::Moderate => 50_000,
            PerformanceProfile::HighPerformance => 100_000,
        }
    }
}

/// Detects hardware capability profile based on available
/// RAM (GB), disk space (GB) for `target_path` and CPU cores.
pub fn detect_profile_for_path<P: AsRef<Path>>(target_path: P) -> PerformanceProfile {
    let mut sys = sysinfo::System::new_all();
    sys.refresh_memory();

    let free_ram_bg = sys.available_memory() as f64 / (1024.0 * 1024.0 * 1024.0);

    let logical_cpus = thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1);

    let target_canonical = target_path
        .as_ref()
        .canonicalize()
        .unwrap_or_else(|_| target_path.as_ref().to_path_buf());

    let disks = Disks::new_with_refreshed_list();
    let available_disk_db = disks
        .list()
        .iter()
        .filter(|disk| target_canonical.starts_with(disk.mount_point()))
        .max_by_key(|disk| disk.mount_point().components().count())
        .map(Disk::available_space)
        .unwrap_or_else(|| {
            disks
                .list()
                .iter()
                .map(|d| d.available_space())
                .max()
                .unwrap_or(0)
        }) as f64
        / (1024.0 * 1024.0 * 1024.0);

    evaluate_profile(free_ram_bg, available_disk_db, logical_cpus)
}

/// Helper function to categorize system metrics into a `PerformanceProfile`
pub fn evaluate_profile(
    free_ram_bg: f64,
    available_disk_db: f64,
    cpu_cores: usize,
) -> PerformanceProfile {
    if free_ram_bg >= 8.0 && available_disk_db >= 30.0 && cpu_cores >= 4 {
        PerformanceProfile::HighPerformance
    } else if free_ram_bg >= 4.0 && available_disk_db >= 15.0 && cpu_cores >= 2 {
        PerformanceProfile::Moderate
    } else {
        PerformanceProfile::Conservative
    }
}

/// Global convenience detector using current directory `.`
pub fn detect_profile() -> PerformanceProfile {
    detect_profile_for_path(".")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evaluate_profile_high_performance() {
        assert_eq!(
            evaluate_profile(16.0, 50.0, 8),
            PerformanceProfile::HighPerformance
        );
        assert_eq!(
            evaluate_profile(8.0, 30.0, 4),
            PerformanceProfile::HighPerformance
        );
    }

    #[test]
    fn test_evaluate_profile_moderate() {
        assert_eq!(evaluate_profile(6.0, 20.0, 4), PerformanceProfile::Moderate);
        assert_eq!(evaluate_profile(4.0, 15.0, 2), PerformanceProfile::Moderate);
    }

    #[test]
    fn test_evaluate_profile_conservative() {
        // Low RAM
        assert_eq!(
            evaluate_profile(3.5, 50.0, 8),
            PerformanceProfile::Conservative
        );
        // Low Disk
        assert_eq!(
            evaluate_profile(16.0, 10.0, 8),
            PerformanceProfile::Conservative
        );
        // Single core
        assert_eq!(
            evaluate_profile(16.0, 50.0, 1),
            PerformanceProfile::Conservative
        );
    }

    #[test]
    fn test_detect_profile_runs_without_panic() {
        let profile = detect_profile();
        assert!(
            profile == PerformanceProfile::Conservative
                || profile == PerformanceProfile::Moderate
                || profile == PerformanceProfile::HighPerformance
        );
    }
}
