use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::collections::HashMap;

fn benchmark(c: &mut Criterion) {
    let sizes = [10, 50, 100, 500, 1_000, 5_000, 10_000, 100_000, 1_000_000];

    for &size in &sizes {
        let vec: Vec<usize> = (0..size).collect();
        let mut map: HashMap<usize, usize> = HashMap::new();
        for &item in &vec {
            map.insert(item, item);
        }

        c.bench_function(&format!("vec_{}", size), |b| {
            b.iter(|| {
                for &item in black_box(&vec) {
                    black_box(vec.iter().find(|&&x| x == item));
                }
            })
        });

        c.bench_function(&format!("map_{}", size), |b| {
            b.iter(|| {
                for &item in black_box(&vec) {
                    black_box(map.get(&item));
                }
            })
        });
    }
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
