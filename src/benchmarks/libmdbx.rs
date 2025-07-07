use crate::common::{DATA_SIZE, generate_test_data};
use std::{hint::black_box, time::Instant};
use tempfile::tempdir;

pub fn benchmark_libmdbx() {
    let dir = tempdir().unwrap();

    let db = libmdbx::Database::<libmdbx::WriteMap>::open_with_options(
        &dir,
        libmdbx::DatabaseOptions {
            mode: libmdbx::Mode::ReadWrite(libmdbx::ReadWriteOptions {
                min_size: Some(1024 * 1024 * 1024),       // 1GB
                sync_mode: libmdbx::SyncMode::SafeNoSync,
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
