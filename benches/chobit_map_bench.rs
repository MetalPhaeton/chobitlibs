use criterion::{black_box, criterion_group, criterion_main, Criterion};
use chobit::chobit_map::ChobitMap;

const MAX: u64 = 10000;

fn chobit_map_bench(c: &mut Criterion) {
    let mut map = ChobitMap::<i32>::new(1024);

    c.bench_function("chobit_map_bench", |b| b.iter(|| {
        for i in 0..MAX {
            map.add(i, i as i32);
        }

        for i in 0..MAX {
            map.get(i).unwrap();
        }

        for i in 0..MAX {
            map.remove(i).unwrap();
        }
    }));
}

fn rust_hash_map_bench(c: &mut Criterion) {
    use std::collections::HashMap;

    let mut map = HashMap::<u64, i32>::new();

    c.bench_function("rust_hash_map_bench", |b| b.iter(|| {
        for i in 0..MAX {
            map.insert(i, i as i32);
        }

        for i in 0..MAX {
            map.get(&i).unwrap();
        }

        for i in 0..MAX {
            map.remove(&i).unwrap();
        }
    }));
}

criterion_group!(chobit_map_benches, chobit_map_bench, rust_hash_map_bench);
criterion_main!(chobit_map_benches);
