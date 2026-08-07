use crate::etl::monitor::detect_profile;

mod etl;
mod models;

fn main() {
    let profile = detect_profile();
    println!("Detected Hardware Performance Profile: {:?}", profile);
    println!(
        "  Max Concurrent Downloads: {}",
        profile.max_concurrent_downloads()
    );
    println!("  CSV Parsing Workers: {}", profile.csv_parsing_workers());
    println!("  DB Batch Size: {}", profile.db_batch_size());
}
