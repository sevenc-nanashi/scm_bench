use criterion::{black_box, criterion_group, criterion_main, Criterion};

const BATCH_SIZE: usize = 1024;

fn benchmark(c: &mut Criterion) {
    let mut g = c.benchmark_group("Vec Insertion Methods");
    for size_b in 10..=21 {
        let size = 1 << size_b;

        g.throughput(criterion::Throughput::Elements(size as u64));
        g.bench_with_input(
            criterion::BenchmarkId::new("push", size),
            &size,
            |b, &size| {
                b.iter(|| {
                    let mut vec = Vec::with_capacity(size);
                    for i in 0..size {
                        vec.push(black_box(i));
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
                        for i in 0..size {
                            ptr[i].write(black_box(i));
                        }
                        vec.set_len(size);
                    }
                    black_box(vec);
                });
            },
        );
    }
    g.finish();
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
