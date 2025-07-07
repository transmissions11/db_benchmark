mod benchmarks;
mod common;

use crate::benchmarks::{hashmap, libmdbx, redb, sled};
use crate::common::DATA_SIZE;

fn main() {
    println!("Benchmarking {DATA_SIZE} batch writes and reads of random u64 key-value pairs\n");
    hashmap::benchmark_hashmap_noncrypto();
    println!("");
    hashmap::benchmark_hashmap();
    println!("");
    libmdbx::benchmark_libmdbx();
    println!("");
    redb::benchmark_redb();
    println!("");
    sled::benchmark_sled();
}
