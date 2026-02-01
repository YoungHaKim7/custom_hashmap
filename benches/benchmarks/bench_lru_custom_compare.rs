use criterion::{Criterion, criterion_group};

use custom_hashmap::LRUCache;

use std::{collections::HashMap, hint::black_box};

pub fn bench_lru_compare(c: &mut Criterion) {
    let mut group = c.benchmark_group("lookup test");

    group.bench_function("custom LRU mixed", |b| {
        b.iter(|| {
            let mut lru = LRUCache::new(128);
            for i in 0..10_000 {
                lru.insert(i % 256, i);
                black_box(lru.get(&(i % 128)));
            }
        })
    });

    group.bench_function("std HashMap LRU", |b| {
        b.iter(|| {
            let mut map = HashMap::new();
            let mut order = std::collections::VecDeque::new();

            for i in 0..10_000 {
                let k = i % 256;
                if !map.contains_key(&k) {
                    order.push_front(k);
                }
                map.insert(k, i);
                black_box(map.get(&(i % 128)));
            }
        })
    });

    group.finish();
}

criterion_group!(benches, bench_lru_compare);
