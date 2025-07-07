use rand::Rng;
use std::collections::HashMap;
use std::time::Instant;
use tempfile::tempdir;

const DATA_SIZE: usize = 100_000;

fn generate_test_data() -> Vec<(u64, u64)> {
    let mut rng = rand::thread_rng();
    let mut data = Vec::new();
    for _ in 0..DATA_SIZE {
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

    println!("Sled: {DATA_SIZE} batch write took {:?}", duration);

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

    println!("Redb: {DATA_SIZE} batch write took {:?}", duration);
}

fn benchmark_hashmap() {
    let data = generate_test_data();

    // Measure start
    let start = Instant::now();
    let mut map = HashMap::new();
    for (key, value) in data {
        map.insert(key, value);
    }
    let duration = start.elapsed();
    // Measure end

    println!("HashMap: {DATA_SIZE} insert took {:?}", duration);
}

fn benchmark_libmdbx() {
    let dir = tempdir().unwrap();

    let db = libmdbx::Database::<libmdbx::NoWriteMap>::open_with_options(
        &dir,
        libmdbx::DatabaseOptions {
            mode: libmdbx::Mode::ReadWrite(libmdbx::ReadWriteOptions {
                sync_mode: libmdbx::SyncMode::UtterlyNoSync,
                min_size: Some(1 * 1024 * 1024 * 1024), // 1GB
                max_size: Some(1000 * 1024 * 1024 * 1024), // 1TB
                ..Default::default()
            }),
            ..Default::default()
        },
    )
    .unwrap();

    let data = generate_test_data();

    let txn = db.begin_rw_txn().unwrap();
    let table = txn.open_table(None).unwrap();

    // Measure start
    let start = Instant::now();
    for (key, value) in data.iter() {
        txn.put(
            &table,
            &key.to_be_bytes(),
            &value.to_be_bytes(),
            libmdbx::WriteFlags::empty(),
        )
        .unwrap();
    }
    txn.commit().unwrap();
    let duration = start.elapsed();
    // Measure end

    println!("Libmdbx: {DATA_SIZE} insert took {:?}", duration);
}

fn main() {
    println!("Benchmarking {DATA_SIZE} batch writes of random u64 key-value pairs\n");
    benchmark_sled();
    benchmark_redb();
    benchmark_hashmap();
    benchmark_libmdbx();
}
