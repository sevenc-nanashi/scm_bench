use criterion::{black_box, criterion_group, criterion_main, Criterion};


const BATCH_SIZE: usize = 1024;

fn push_benchmark(c: &mut Criterion) {
    c.bench_function("push", |b| {
        b.iter(|| {
            let mut vec = Vec::with_capacity(BATCH_SIZE);
            for i in 0..BATCH_SIZE {
                vec.push(black_box(i));
            }
        })
    });
}

fn spare_capacity_mut_benchmark(c: &mut Criterion) {
    c.bench_function("spare_capacity_mut", |b| {
        b.iter(|| {
            let mut vec = Vec::with_capacity(BATCH_SIZE);
            let spare = vec.spare_capacity_mut();
            for i in 0..BATCH_SIZE {
                spare[i].write(black_box(i));
            }
            unsafe {
                vec.set_len(BATCH_SIZE);
            }
        })
    });
}

criterion_group!(benches, push_benchmark, spare_capacity_mut_benchmark);
criterion_main!(benches);
