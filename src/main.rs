mod benchmarks;
mod common;

use crate::benchmarks::{hashmap, libmdbx, redb, sled};
use crate::common::DATA_SIZE;

fn main() {
    println!("Benchmarking {DATA_SIZE} batch writes and reads of random u64 key-value pairs\n");
    sled::benchmark_sled();
    redb::benchmark_redb();
    hashmap::benchmark_hashmap();
    hashmap::benchmark_hashmap_noncrypto();
    libmdbx::benchmark_libmdbx();
}
