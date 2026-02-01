use criterion::{Criterion, criterion_group};
use custom_hashmap::CustomHashMap;
use std::hint::black_box;

pub fn bench_lookup_compare(c: &mut Criterion) {
    let mut group = c.benchmark_group("lookup test");

    group.bench_function("custom HashMap lookup", |b| {
        b.iter(|| {
            let mut map = CustomHashMap::new();
            for i in 0..1000 {
                map.insert(i, i);
            }
            for i in 0..1000 {
                black_box(map.get(&i));
            }
        })
    });

    group.bench_function("std HashMap lookup", |b| {
        b.iter(|| {
            let mut map = std::collections::HashMap::new();
            for i in 0..1000 {
                map.insert(i, i);
            }
            for i in 0..1000 {
                black_box(map.get(&i));
            }
        })
    });

    group.finish();
}

criterion_group!(benches, bench_lookup_compare);
