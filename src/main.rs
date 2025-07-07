use rand::Rng;
use std::collections::HashMap;
use std::time::Instant;
use tempfile::tempdir;

fn generate_test_data() -> Vec<(u64, u64)> {
    let mut rng = rand::thread_rng();
    let mut data = Vec::with_capacity(100_000);
    for _ in 0..100_000 {
        data.push((rng.r#gen::<u64>(), rng.r#gen::<u64>()));
    }
    data
}

fn benchmark_sled() {
    let config = sled::Config::new()
        .cache_capacity(1024 * 1024 * 1024) // 1GB
        .temporary(true)
        .flush_every_ms(None) // Disable automatic flushing.
        .mode(sled::Mode::HighThroughput);
    let db = config.open().unwrap();
    let data = generate_test_data();

    // Measure start
    let start = Instant::now();
    let mut batch = sled::Batch::default();
    for (key, value) in &data {
        batch.insert(&key.to_be_bytes(), &value.to_be_bytes());
    }
    db.apply_batch(batch).unwrap();
    let duration = start.elapsed();
    // Measure end

    println!("Sled: 100k batch write took {:?}", duration);

    // Measure start
    let start = Instant::now();
    db.flush().unwrap();
    let duration = start.elapsed();
    // Measure end

    println!("Sled: flush took {:?}", duration);
}

fn benchmark_redb() {
    const TABLE: redb::TableDefinition<u64, u64> = redb::TableDefinition::new("bench");

    let temp_dir = tempdir().expect("Failed to create tempdir");
    let db_path = temp_dir.path().join("redb_bench.db");
    let db = redb::Database::create(&db_path).unwrap();
    let data = generate_test_data();

    // Measure start
    let start = Instant::now();
    let mut write_txn = db.begin_write().unwrap();
    write_txn.set_durability(redb::Durability::Eventual);
    {
        let mut table = write_txn.open_table(TABLE).unwrap();
        for (key, value) in data {
            table.insert(key, value).unwrap();
        }
    }
    write_txn.commit().unwrap();
    let duration = start.elapsed();
    // Measure end

    println!("Redb: 100k batch write took {:?}", duration);
}

fn benchmark_hashmap() {
    let data = generate_test_data();

    // Measure start
    let start = Instant::now();
    let mut map = HashMap::with_capacity(100_000);
    for (key, value) in data {
        map.insert(key, value);
    }
    let duration = start.elapsed();
    // Measure end

    println!("HashMap: 100k insert took {:?}", duration);
}

fn main() {
    println!("Benchmarking 100k batch writes of random u64 key-value pairs\n");
    benchmark_sled();
    benchmark_redb();
    benchmark_hashmap();
}
