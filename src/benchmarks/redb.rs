use crate::common::{DATA_SIZE, generate_test_data};
use std::time::Instant;
use tempfile::tempdir;

pub fn benchmark_redb() {
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
