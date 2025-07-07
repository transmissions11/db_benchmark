mod benchmarks;
mod common;

use crate::benchmarks::{hashmap, libmdbx, redb, sled};
use crate::common::DATA_SIZE;

fn main() {
    println!("Benchmarking {DATA_SIZE} batch writes and reads of random u64 key-value pairs\n");
    hashmap::benchmark_hashmap_noncrypto();
    hashmap::benchmark_hashmap();
    libmdbx::benchmark_libmdbx();
    redb::benchmark_redb();
    sled::benchmark_sled();
}
