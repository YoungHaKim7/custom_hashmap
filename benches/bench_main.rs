use criterion::criterion_main;

mod benchmarks;

criterion_main!(
    benchmarks::std_hashmap_my_hashmap::benches,
    benchmarks::bench_insert_std_vs_customhashmap::benches,
    benchmarks::bench_lookup_custom::benches,
    benchmarks::bench_lru_custom_compare::benches,
);
