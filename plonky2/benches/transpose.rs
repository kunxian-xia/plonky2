mod allocator;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use plonky2::field::goldilocks_field::GoldilocksField;
use plonky2::field::types::Sample;
use plonky2::util::transpose;

fn criterion_benchmark(c: &mut Criterion) {
    type F = GoldilocksField;

    // In practice, for the matrices we care about, each row is associated with a polynomial of
    // degree 2^13, and has been low-degree extended to a length of 2^16.
    // const WIDTH: usize = 1 << 16;

    // We have matrices with various numbers of polynomials. For example, the witness matrix
    // involves 100+ polynomials.
    for lg_width in [16, 17, 18, 19, 20, 21, 22, 23] {
        let mut group = c.benchmark_group(&format!("transpose<rows=2^{}>", lg_width));
        for height in [5, 50, 100, 150, 200, 400, 600, 700] {
            group.bench_with_input(BenchmarkId::from_parameter(height), &height, |b, _| {
                let matrix = (0..height)
                    .map(|_| F::rand_vec(1 << lg_width))
                    .collect::<Vec<_>>();
                b.iter(|| transpose(&matrix));
            });
        }
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
