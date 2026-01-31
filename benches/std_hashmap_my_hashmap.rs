use criterion::{Criterion, criterion_group, criterion_main};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use custom_hashmap::AssemblyHash;

fn all(c: &mut Criterion) {
    c.bench_function("std Hashmap test", |b| {
        b.iter(|| std::collections::HashMap::<u32, u32>::new());
    });
    c.bench_function("std Hash", |b| {
        b.iter(|| {
            let mut hasher = DefaultHasher::new();
            20u32.hash(&mut hasher);
            hasher.finish()
        });
    });
    c.bench_function("my assembly hash", |b| {
        b.iter(|| 20u32.assembly_hash());
    });
}

criterion_group!(g, all);
criterion_main!(g);
