use crate::common::{DATA_SIZE, generate_large_initial_data, generate_mixed_update_data};
use std::{hint::black_box, time::Instant};
use tempfile::tempdir;

pub fn benchmark_libmdbx() {
    let dir = tempdir().unwrap();

    let db = libmdbx::Database::<libmdbx::WriteMap>::open_with_options(
        &dir,
        libmdbx::DatabaseOptions {
            mode: libmdbx::Mode::ReadWrite(libmdbx::ReadWriteOptions {
                sync_mode: libmdbx::SyncMode::SafeNoSync,
                max_size: Some(100 * 1024 * 1024 * 1024), // 100GB
                ..Default::default()
            }),
            ..Default::default()
        },
    )
    .unwrap();

    // Generate large initial dataset
    let initial_data = generate_large_initial_data();

    // Measure initial population
    let start = Instant::now();
    let txn = db.begin_rw_txn().unwrap();
    let table = txn.open_table(None).unwrap();
    for (key, value) in initial_data.iter() {
        txn.put(
            &table,
            &key.to_be_bytes(),
            &value.to_be_bytes(),
            libmdbx::WriteFlags::empty(),
        )
        .unwrap();
    }
    txn.commit().unwrap();
    let initial_write_duration = start.elapsed();

    println!(
        "Libmdbx: Initial population of {} entries took {:?}",
        initial_data.len(),
        initial_write_duration
    );

    // Generate mixed update/insert data
    let mixed_data = generate_mixed_update_data(&initial_data);

    // Measure mixed writes (updates + inserts)
    let start = Instant::now();
    let txn = db.begin_rw_txn().unwrap();
    let table = txn.open_table(None).unwrap();
    for (key, value) in mixed_data.iter() {
        txn.put(
            &table,
            &key.to_be_bytes(),
            &value.to_be_bytes(),
            libmdbx::WriteFlags::empty(),
        )
        .unwrap();
    }
    txn.commit().unwrap();
    let mixed_write_duration = start.elapsed();

    // Measure read after mixed writes
    let start = Instant::now();
    let txn = db.begin_ro_txn().unwrap();
    let table = txn.open_table(None).unwrap();
    for (key, _) in mixed_data.iter() {
        let _: Option<Vec<u8>> = black_box(txn.get(&table, &key.to_be_bytes()).unwrap());
    }
    let read_duration = start.elapsed();

    let sync_start = Instant::now();
    db.sync(true).unwrap();
    let sync_duration = sync_start.elapsed();

    println!(
        "Libmdbx: Mixed writes ({} ops) took {:?}, full read took {:?}, sync took {:?}",
        DATA_SIZE, mixed_write_duration, read_duration, sync_duration
    );
}
