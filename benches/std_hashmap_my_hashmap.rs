use criterion::{Criterion, criterion_group, criterion_main};

use custom_hashmap::AssemblyHash;

fn all(c: &mut Criterion) {
    c.bench_function("std Hashmap test", |b| {
        b.iter(|| std::collections::HashMap::<u32, u32>::new());
    });
    c.bench_function("my assembly hash", |b| {
        b.iter(|| 20u32.assembly_hash());
    });
}

criterion_group!(g, all);
criterion_main!(g);
