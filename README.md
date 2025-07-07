# kv-benchmarks

Some contrived benchmarking of embedded kv dbs.

## Results

On my M1 Pro, 32 GB as of commit `3ce0efabba9eab9710492a344ba4cc8ac5e8246d`:

```
❯ cargo run --profile release
   Compiling db_benchmark v0.1.0 (/Users/transmission11/Desktop/db_benchmark)
    Finished `release` profile [optimized] target(s) in 1.50s
     Running `target/release/db_benchmark`
Benchmarking 100000 batch writes and reads of random u64 key-value pairs

HashMap non-crypto: Initial population of 10000000 entries took 316.819166ms
HashMap non-crypto: Mixed writes (100000 ops) took 2.727708ms, full read took 1.280458ms

HashMap: Initial population of 10000000 entries took 796.536958ms
HashMap: Mixed writes (100000 ops) took 9.60275ms, full read took 7.293417ms

Libmdbx: Initial population of 10000000 entries took 9.4063385s
Libmdbx: Mixed writes (100000 ops) took 917.472125ms, full read took 110.535958ms, sync took 14.615667ms

Redb: Initial population of 10000000 entries took 28.68567575s
Redb: Mixed writes (100000 ops) took 2.074889458s, full read took 99.73575ms

Sled: Initial population of 10000000 entries took 59.849701125s
Sled: Mixed writes (100000 ops) took 831.914083ms, full read took 1.124996416s, flush took 155.583µs
```
