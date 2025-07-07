use rand::Rng;
use std::time::Instant;

fn generate_test_data() -> Vec<(u64, u64)> {
    let mut rng = rand::thread_rng();
    let mut data = Vec::with_capacity(100_000);
    for _ in 0..100_000 {
        data.push((rng.r#gen::<u64>(), rng.r#gen::<u64>()));
    }
    data
}

fn benchmark_sled() -> std::io::Result<()> {
    let config = sled::Config::new()
        .cache_capacity(1024 * 1024 * 1024) // 1GB
        .temporary(true)
        .flush_every_ms(None) // Disable automatic flushing.
        .mode(sled::Mode::HighThroughput);
    let db = config.open()?;
    let data = generate_test_data();

    // Measure start
    let start = Instant::now();
    let mut batch = sled::Batch::default();
    for (key, value) in &data {
        batch.insert(&key.to_be_bytes(), &value.to_be_bytes());
    }
    db.apply_batch(batch)?;
    let duration = start.elapsed();
    // Measure end

    println!("Sled: 100k batch write took {:?}", duration);

    // Measure start
    let start = Instant::now();
    db.flush()?;
    let duration = start.elapsed();
    // Measure end

    println!("Sled: flush took {:?}", duration);

    std::fs::remove_dir_all("sled_bench").ok();
    Ok(())
}

fn benchmark_redb() -> Result<(), redb::Error> {
    use redb::{Database, TableDefinition};

    const TABLE: TableDefinition<u64, u64> = TableDefinition::new("bench");

    std::fs::remove_file("redb_bench.db").ok();
    let db = Database::create("redb_bench.db")?;
    let data = generate_test_data();

    // Measure start
    let start = Instant::now();
    let mut write_txn = db.begin_write()?;
    write_txn.set_durability(redb::Durability::Eventual);
    {
        let mut table = write_txn.open_table(TABLE)?;
        for (key, value) in data {
            table.insert(key, value)?;
        }
    }
    write_txn.commit()?;
    let duration = start.elapsed();
    // Measure end

    println!("Redb: 100k batch write took {:?}", duration);

    std::fs::remove_file("redb_bench.db").ok();
    Ok(())
}

fn main() {
    println!("Benchmarking 100k batch writes of random u64 key-value pairs\n");

    if let Err(e) = benchmark_sled() {
        eprintln!("Sled benchmark failed: {}", e);
    }

    if let Err(e) = benchmark_redb() {
        eprintln!("Redb benchmark failed: {}", e);
    }
}
