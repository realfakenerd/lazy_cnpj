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

pub fn detect_profile() -> PerformanceProfile {
    let mut sys = sysinfo::System::new_all();
    sys.refresh_memory();

    let free_ram_bg = sys.available_memory() as f64 / (1024.0 * 1024.0 * 1024.0);

    let disk = Disks::new_with_refreshed_list();
    let available_disk_db = disk
        .list()
        .iter()
        .map(Disk::available_space)
        .max()
        .unwrap_or(0) as f64
        / (1024.0 * 1024.0 * 1024.0);

    if free_ram_bg > 8.0 && available_disk_db > 30.0 {
        PerformanceProfile::HighPerformance
    } else if free_ram_bg > 4.0 && available_disk_db > 15.0 {
        PerformanceProfile::Moderate
    } else {
        PerformanceProfile::Conservative
    }
}
