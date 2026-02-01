use criterion::{Criterion, criterion_group};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use custom_hashmap::AssemblyHash;

pub fn assembly_hash_compare(c: &mut Criterion) {
    let mut group = c.benchmark_group("assembly hash test");
    group.bench_function("std Hashmap test", |b| {
        b.iter(|| std::collections::HashMap::<u32, u32>::new());
    });
    group.bench_function("std Hash", |b| {
        b.iter(|| {
            let mut hasher = DefaultHasher::new();
            20u32.hash(&mut hasher);
            hasher.finish()
        });
    });
    group.bench_function("my assembly hash", |b| {
        b.iter(|| 20u32.assembly_hash());
    });
    group.finish();
}

criterion_group!(benches, assembly_hash_compare);
