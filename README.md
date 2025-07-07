# db_benchmark

Testing some embedded dbs in Rust.

```sh
❯ cargo run --profile release
   Compiling db_benchmark v0.1.0 (/Users/transmission11/Desktop/db_benchmark)
    Finished `release` profile [optimized] target(s) in 0.54s
     Running `target/release/db_benchmark`
Benchmarking 100k batch writes of random u64 key-value pairs

Sled: 100k batch write took 273.602459ms
Sled: flush took 96.167µs
Redb: 100k batch write took 118.479791ms
```
