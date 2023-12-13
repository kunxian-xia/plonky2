mod allocator;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use plonky2::field::goldilocks_field::GoldilocksField;
use plonky2::field::polynomial::PolynomialCoeffs;
use plonky2::field::types::Field;
use tynm::type_name;

pub(crate) fn bench_ffts<F: Field>(c: &mut Criterion) {
    let mut group = c.benchmark_group(&format!("fft<{}>", type_name::<F>()));

    for size_log in [19, 20, 21, 22, 23] {
        let size = 1 << size_log;
        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, _| {
            let coeffs = PolynomialCoeffs::new(F::rand_vec(size));
            b.iter(|| coeffs.clone().fft_with_options(None, None));
        });
    }
}

pub(crate) fn bench_ldes<F: Field>(c: &mut Criterion, rate_bits: usize) {
    // const RATE_BITS: usize = 3;

    let mut group = c.benchmark_group(&format!("lde<{}>", type_name::<F>()));

    for size_log in [20, 21, 22, 23] {
        let orig_size = 1 << (size_log - rate_bits);
        let lde_size = 1 << size_log;

        group.bench_with_input(BenchmarkId::from_parameter(lde_size), &lde_size, |b, _| {
            let coeffs = PolynomialCoeffs::new(F::rand_vec(orig_size));
            b.iter(|| {
                let padded_coeffs = coeffs.lde(rate_bits);
                padded_coeffs.fft_with_options(Some(rate_bits), None)
            });
        });
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    bench_ffts::<GoldilocksField>(c);
    bench_ldes::<GoldilocksField>(c, 1); // hermez
    bench_ldes::<GoldilocksField>(c, 2);
    bench_ldes::<GoldilocksField>(c, 3); // recursion and scroll
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
