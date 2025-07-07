use crate::common::{DATA_SIZE, generate_large_initial_data, generate_mixed_update_data};
use std::time::Instant;

pub fn benchmark_hashmap() {
    // Generate large initial dataset
    let initial_data = generate_large_initial_data();

    // Measure initial population
    let start = Instant::now();
    let mut map = std::collections::HashMap::new();
    for (key, value) in &initial_data {
        map.insert(*key, *value);
    }
    let initial_write_duration = start.elapsed();

    println!(
        "HashMap: Initial population of {} entries took {:?}",
        initial_data.len(),
        initial_write_duration
    );

    // Generate mixed update/insert data
    let mixed_data = generate_mixed_update_data(&initial_data);

    // Measure mixed writes (updates + inserts)
    let start = Instant::now();
    for (key, value) in &mixed_data {
        map.insert(*key, *value);
    }
    let mixed_write_duration = start.elapsed();

    // Measure read after mixed writes
    let start = Instant::now();
    for (key, _) in &mixed_data {
        let _ = map.get(key);
    }
    let read_duration = start.elapsed();

    println!(
        "HashMap: Mixed writes ({} ops) took {:?}, full read took {:?}",
        DATA_SIZE, mixed_write_duration, read_duration
    );
}

pub fn benchmark_hashmap_noncrypto() {
    // Generate large initial dataset
    let initial_data = generate_large_initial_data();

    // Measure initial population
    let start = Instant::now();
    let mut map = hashbrown::HashMap::new();
    for (key, value) in &initial_data {
        map.insert(*key, *value);
    }
    let initial_write_duration = start.elapsed();

    println!(
        "HashMap non-crypto: Initial population of {} entries took {:?}",
        initial_data.len(),
        initial_write_duration
    );

    // Generate mixed update/insert data
    let mixed_data = generate_mixed_update_data(&initial_data);

    // Measure mixed writes (updates + inserts)
    let start = Instant::now();
    for (key, value) in &mixed_data {
        map.insert(*key, *value);
    }
    let mixed_write_duration = start.elapsed();

    // Measure read after mixed writes
    let start = Instant::now();
    for (key, _) in &mixed_data {
        let _ = map.get(key);
    }
    let read_duration = start.elapsed();

    println!(
        "HashMap non-crypto: Mixed writes ({} ops) took {:?}, full read took {:?}",
        DATA_SIZE, mixed_write_duration, read_duration
    );
}
