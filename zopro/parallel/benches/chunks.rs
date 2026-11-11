use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use rayon::{prelude::*, ThreadPoolBuilder};
use std::time::Duration;
use zopro_parallel::for_each_chunk_mut;

const LEN: usize = 1 << 16;
const CHUNK: usize = 256;

fn kernel(start: usize, values: &mut [u64], irregular: bool) {
    let rounds = if irregular && (start / CHUNK).is_multiple_of(8) {
        64
    } else {
        8
    };
    for (offset, value) in values.iter_mut().enumerate() {
        let mut x = *value ^ (start + offset) as u64;
        for _ in 0..rounds {
            x = x.wrapping_mul(0x9e37_79b9_7f4a_7c15).rotate_left(17);
        }
        *value = x;
    }
}

fn execute(values: &mut [u64], bounded: bool, irregular: bool) {
    if bounded {
        for_each_chunk_mut(values, CHUNK, |start, chunk| {
            kernel(start, chunk, irregular);
        });
    } else {
        values
            .par_chunks_mut(CHUNK)
            .enumerate()
            .for_each(|(index, chunk)| kernel(index * CHUNK, chunk, irregular));
    }
}

fn chunks(c: &mut Criterion) {
    for workers in [1, 4, 16, 32] {
        let pool = ThreadPoolBuilder::new()
            .num_threads(workers)
            .build()
            .unwrap();
        for irregular in [false, true] {
            let mut expected = vec![1_u64; LEN];
            for (index, chunk) in expected.chunks_mut(CHUNK).enumerate() {
                kernel(index * CHUNK, chunk, irregular);
            }
            let label = if irregular { "irregular" } else { "regular" };
            let mut group = c.benchmark_group(format!("{label}/{workers}-workers"));
            group.throughput(Throughput::Elements(LEN as u64));
            group.sample_size(20);
            group.warm_up_time(Duration::from_millis(100));
            group.measurement_time(Duration::from_millis(300));
            for bounded in [false, true] {
                let mut values = vec![1_u64; LEN];
                pool.install(|| execute(&mut values, bounded, irregular));
                assert_eq!(values, expected);
                values.fill(1);
                let name = if bounded { "bounded" } else { "rayon" };
                group.bench_with_input(BenchmarkId::new(name, CHUNK), &bounded, |b, &bounded| {
                    b.iter(|| {
                        pool.install(|| execute(&mut values, bounded, irregular));
                        black_box(&values);
                    });
                });
            }
            group.finish();
        }
    }
}

criterion_group!(benches, chunks);
criterion_main!(benches);
