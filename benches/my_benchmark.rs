use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark(c: &mut Criterion) {
    let mut g = c.benchmark_group("Vec Initialization Methods");
    for size_b in 10..=21 {
        let size = 1 << size_b;

        let base_data: Vec<usize> = (0..size).collect();

        g.throughput(criterion::Throughput::Elements(size as u64));
        g.bench_with_input(
            criterion::BenchmarkId::new("push", size),
            &size,
            |b, &size| {
                b.iter(|| {
                    let mut vec = Vec::with_capacity(size);
                    for (_, i) in base_data.iter().enumerate() {
                        vec.push(black_box(i * i));
                    }
                    black_box(vec);
                });
            },
        );
        g.bench_with_input(
            criterion::BenchmarkId::new("spare_capacity_mut", size),
            &size,
            |b, &size| {
                b.iter(|| {
                    let mut vec = Vec::with_capacity(size);
                    unsafe {
                        let ptr = vec.spare_capacity_mut();
                        for (i, d) in base_data.iter().enumerate() {
                            ptr[i].write(black_box(d * d));
                        }
                        vec.set_len(size);
                    }
                    black_box(vec);
                });
            },
        );
        g.bench_with_input(
            criterion::BenchmarkId::new("set_len", size),
            &size,
            |b, &size| {
                b.iter(|| {
                    let mut vec = Vec::with_capacity(size);
                    unsafe {
                        vec.set_len(size);
                        for (i, d) in base_data.iter().enumerate() {
                            vec[i] = black_box(d * d);
                        }
                    }
                    black_box(vec);
                });
            },
        );
        g.bench_with_input(
            criterion::BenchmarkId::new("map", size),
            &size,
            |b, &_size| {
                b.iter(|| {
                    let vec: Vec<usize> = base_data.iter().map(|&x| black_box(x * x)).collect();
                    black_box(vec);
                });
            },
        );
    }
    g.finish();
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
