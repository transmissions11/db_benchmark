use crate::common::{DATA_SIZE, generate_large_initial_data, generate_mixed_update_data};
use std::time::Instant;

pub fn benchmark_sled() {
    let config = sled::Config::new()
        .cache_capacity(1024 * 1024 * 1024) // 1GB
        .temporary(true)
        .flush_every_ms(None) // Disable automatic flushing.
        .mode(sled::Mode::HighThroughput);
    let db = config.open().unwrap();

    // Generate large initial dataset
    let initial_data = generate_large_initial_data();

    // Measure initial population
    let start = Instant::now();
    let mut batch = sled::Batch::default();
    for (key, value) in &initial_data {
        batch.insert(&key.to_be_bytes(), &value.to_be_bytes());
    }
    db.apply_batch(batch).unwrap();
    let initial_write_duration = start.elapsed();

    println!(
        "Sled: Initial population of {} entries took {:?}",
        initial_data.len(),
        initial_write_duration
    );

    // Generate mixed update/insert data
    let mixed_data = generate_mixed_update_data(&initial_data);

    // Measure mixed writes (updates + inserts)
    let start = Instant::now();
    let mut batch = sled::Batch::default();
    for (key, value) in &mixed_data {
        batch.insert(&key.to_be_bytes(), &value.to_be_bytes());
    }
    db.apply_batch(batch).unwrap();
    let mixed_write_duration = start.elapsed();

    // Measure read after mixed writes
    let start = Instant::now();
    for (key, _) in &mixed_data {
        let _ = db.get(&key.to_be_bytes()).unwrap();
    }
    let read_duration = start.elapsed();

    // Measure flush
    let flush_start = Instant::now();
    db.flush().unwrap();
    let flush_duration = flush_start.elapsed();

    println!(
        "Sled: Mixed writes ({} ops) took {:?}, full read took {:?}, flush took {:?}",
        DATA_SIZE, mixed_write_duration, read_duration, flush_duration
    );
}
