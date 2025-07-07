use crate::common::{DATA_SIZE, generate_test_data};
use std::time::Instant;

pub fn benchmark_sled() {
    let config = sled::Config::new()
        .cache_capacity(1024 * 1024 * 1024) // 1GB
        .temporary(true)
        .flush_every_ms(None) // Disable automatic flushing.
        .mode(sled::Mode::HighThroughput);
    let db = config.open().unwrap();
    let data = generate_test_data();

    // Measure write
    let start = Instant::now();
    let mut batch = sled::Batch::default();
    for (key, value) in &data {
        batch.insert(&key.to_be_bytes(), &value.to_be_bytes());
    }
    db.apply_batch(batch).unwrap();
    let write_duration = start.elapsed();

    // Measure read
    let start = Instant::now();
    for (key, _) in &data {
        let _ = db.get(&key.to_be_bytes()).unwrap();
    }
    let read_duration = start.elapsed();

    // Measure flush
    let start = Instant::now();
    db.flush().unwrap();
    let flush_duration = start.elapsed();

    println!(
        "Sled: {DATA_SIZE} batch write took {:?}, read took {:?}, flush took {:?}",
        write_duration, read_duration, flush_duration
    );
}
