use crate::common::{DATA_SIZE, generate_test_data};
use std::time::Instant;

pub fn benchmark_hashmap() {
    let data = generate_test_data();

    // Measure write
    let start = Instant::now();
    let mut map = std::collections::HashMap::new();
    for (key, value) in &data {
        map.insert(*key, *value);
    }
    let write_duration = start.elapsed();

    // Measure read
    let start = Instant::now();
    for (key, _) in &data {
        let _ = map.get(key);
    }
    let read_duration = start.elapsed();

    println!(
        "HashMap: {DATA_SIZE} insert took {:?}, read took {:?}",
        write_duration, read_duration
    );
}

pub fn benchmark_hashmap_noncrypto() {
    let data = generate_test_data();

    // Measure write
    let start = Instant::now();
    let mut map = hashbrown::HashMap::new();
    for (key, value) in &data {
        map.insert(*key, *value);
    }
    let write_duration = start.elapsed();

    // Measure read
    let start = Instant::now();
    for (key, _) in &data {
        let _ = map.get(key);
    }
    let read_duration = start.elapsed();

    println!(
        "HashMap non-crypto: {DATA_SIZE} insert took {:?}, read took {:?}",
        write_duration, read_duration
    );
}
