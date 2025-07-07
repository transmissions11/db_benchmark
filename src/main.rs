use rand::Rng;
use std::{hint::black_box, time::Instant};
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

fn benchmark_hashmap() {
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

fn benchmark_hashmap_noncrypto() {
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

fn benchmark_sled() {
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

    // Flush
    let start = Instant::now();
    db.flush().unwrap();
    let flush_duration = start.elapsed();

    // Measure read
    let start = Instant::now();
    for (key, _) in &data {
        let _ = db.get(&key.to_be_bytes()).unwrap();
    }
    let read_duration = start.elapsed();

    println!(
        "Sled: {DATA_SIZE} batch write took {:?}, flush took {:?}, read took {:?}",
        write_duration, flush_duration, read_duration
    );
}

fn benchmark_redb() {
    const TABLE: redb::TableDefinition<u64, u64> = redb::TableDefinition::new("bench");

    let temp_dir = tempdir().expect("Failed to create tempdir");
    let db_path = temp_dir.path().join("redb_bench.db");
    let db = redb::Database::create(&db_path).unwrap();
    let data = generate_test_data();

    // Measure write
    let start = Instant::now();
    let mut write_txn = db.begin_write().unwrap();
    write_txn.set_durability(redb::Durability::Eventual);
    {
        let mut table = write_txn.open_table(TABLE).unwrap();
        for (key, value) in &data {
            table.insert(*key, *value).unwrap();
        }
    }
    write_txn.commit().unwrap();
    let write_duration = start.elapsed();

    // Measure read
    let start = Instant::now();
    let read_txn = db.begin_read().unwrap();
    let table = read_txn.open_table(TABLE).unwrap();
    for (key, _) in &data {
        let _ = table.get(*key).unwrap();
    }
    let read_duration = start.elapsed();

    println!(
        "Redb: {DATA_SIZE} batch write took {:?}, read took {:?}",
        write_duration, read_duration
    );
}

fn benchmark_libmdbx() {
    let dir = tempdir().unwrap();

    let db = libmdbx::Database::<libmdbx::NoWriteMap>::open_with_options(
        &dir,
        libmdbx::DatabaseOptions {
            mode: libmdbx::Mode::ReadWrite(libmdbx::ReadWriteOptions {
                sync_mode: libmdbx::SyncMode::NoMetaSync,
                min_size: Some(1024 * 1024 * 1024),       // 1GB
                max_size: Some(100 * 1024 * 1024 * 1024), // 100GB
                ..Default::default()
            }),
            ..Default::default()
        },
    )
    .unwrap();

    let data = generate_test_data();

    // Measure write
    let start = Instant::now();
    let txn = db.begin_rw_txn().unwrap();
    let table = txn.open_table(None).unwrap();
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
    let write_duration = start.elapsed();

    // Measure read
    let start = Instant::now();
    let txn = db.begin_ro_txn().unwrap();
    let table = txn.open_table(None).unwrap();
    for (key, _) in data.iter() {
        let _: Option<Vec<u8>> = black_box(txn.get(&table, &key.to_be_bytes()).unwrap());
    }
    let read_duration = start.elapsed();

    let sync_start = Instant::now();
    db.sync(true).unwrap();
    let sync_duration = sync_start.elapsed();

    println!(
        "Libmdbx: {DATA_SIZE} insert took {:?}, read took {:?}, sync took {:?}",
        write_duration, read_duration, sync_duration
    );
}

fn main() {
    println!("Benchmarking {DATA_SIZE} batch writes and reads of random u64 key-value pairs\n");
    benchmark_sled();
    benchmark_redb();
    benchmark_hashmap();
    benchmark_hashmap_noncrypto();
    benchmark_libmdbx();
}
