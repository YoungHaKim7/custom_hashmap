use criterion::{Criterion, criterion_group};
use custom_hashmap::CustomHashMap;

pub fn insert_compare(c: &mut Criterion) {
    let mut group = c.benchmark_group("assembly hash test");
    group.bench_function("custom Hashmap test", |b| {
        b.iter(|| {
            let mut map = CustomHashMap::new();
            for i in 0..1000 {
                map.insert(i, i);
            }
        })
    });
    group.bench_function("std Hashmap insert test", |b| {
        b.iter(|| {
            let mut map = std::collections::HashMap::new();
            for i in 0..1000 {
                map.insert(i, i);
            }
        })
    });
    group.finish();
}

criterion_group!(benches, insert_compare);
