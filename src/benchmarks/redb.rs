use crate::common::{DATA_SIZE, generate_large_initial_data, generate_mixed_update_data};
use std::time::Instant;
use tempfile::tempdir;

pub fn benchmark_redb() {
    const TABLE: redb::TableDefinition<u64, u64> = redb::TableDefinition::new("bench");

    let temp_dir = tempdir().expect("Failed to create tempdir");
    let db_path = temp_dir.path().join("redb_bench.db");
    let db = redb::Database::create(&db_path).unwrap();

    // Generate large initial dataset
    let initial_data = generate_large_initial_data();

    // Measure initial population
    let start = Instant::now();
    let mut write_txn = db.begin_write().unwrap();
    write_txn.set_durability(redb::Durability::Eventual);
    {
        let mut table = write_txn.open_table(TABLE).unwrap();
        for (key, value) in &initial_data {
            table.insert(*key, *value).unwrap();
        }
    }
    write_txn.commit().unwrap();
    let initial_write_duration = start.elapsed();

    println!(
        "Redb: Initial population of {} entries took {:?}",
        initial_data.len(),
        initial_write_duration
    );

    // Generate mixed update/insert data
    let mixed_data = generate_mixed_update_data(&initial_data);

    // Measure mixed writes (updates + inserts)
    let start = Instant::now();
    let mut write_txn = db.begin_write().unwrap();
    write_txn.set_durability(redb::Durability::Eventual);
    {
        let mut table = write_txn.open_table(TABLE).unwrap();
        for (key, value) in &mixed_data {
            table.insert(*key, *value).unwrap();
        }
    }
    write_txn.commit().unwrap();
    let mixed_write_duration = start.elapsed();

    // Measure read after mixed writes
    let start = Instant::now();
    let read_txn = db.begin_read().unwrap();
    let table = read_txn.open_table(TABLE).unwrap();
    for (key, _) in &mixed_data {
        let _ = table.get(*key).unwrap();
    }
    let read_duration = start.elapsed();

    println!(
        "Redb: Mixed writes ({} ops) took {:?}, full read took {:?}",
        DATA_SIZE, mixed_write_duration, read_duration
    );
}
