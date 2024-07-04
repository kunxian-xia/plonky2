mod allocator;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use plonky2::field::goldilocks_field::GoldilocksField;
use plonky2::field::polynomial::PolynomialCoeffs;
use plonky2::field::types::Field;
use plonky2::util::log2_strict;
use plonky2_field::packable::Packable;
use plonky2_field::packed::PackedField;
use plonky2_maybe_rayon::{MaybeParIterMut, ParallelIterator};
use tynm::type_name;

pub(crate) fn bench_ffts<F: Field>(c: &mut Criterion) {
    let mut group = c.benchmark_group(&format!("fft<{}>", type_name::<F>()));

    for size_log in [13, 14, 15, 16] {
        let size = 1 << size_log;
        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, _| {
            let coeffs = PolynomialCoeffs::new(F::rand_vec(size));
            b.iter(|| coeffs.clone().fft_with_options(None, None));
        });
    }
}

pub(crate) fn bench_ldes<F: Field>(c: &mut Criterion, batch_sizes: &[usize]) {
    const RATE_BITS: usize = 1;

    let lg_packed_width = log2_strict(<F as Packable>::Packing::WIDTH);
    println!("lg_packed_width: {}", lg_packed_width);

    let mut group = c.benchmark_group(&format!("lde<{}>", type_name::<F>()));

    for size_log in 15..=23 {
        let orig_size = 1 << size_log;
        let lde_size = 1 << (size_log + RATE_BITS);

        for batch_size in batch_sizes {
            let mut coeffs: Vec<_> = (0..*batch_size)
                .into_iter()
                .map(|_| PolynomialCoeffs::new(F::rand_vec(orig_size)))
                .collect();
            group.bench_with_input(
                BenchmarkId::from_parameter(format!("({},{})", size_log, batch_size)),
                &lde_size,
                |b, _| {
                    b.iter(|| {
                        coeffs.par_iter_mut().for_each(|coeff| {
                            let padded_coeff = coeff.lde(RATE_BITS);
                            padded_coeff.fft_with_options(Some(RATE_BITS), None);
                        })
                    });
                },
            );
        }
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    bench_ldes::<GoldilocksField>(c, &[1, 5, 50, 100, 300]);
    // bench_ffts::<GoldilocksField>(c);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
