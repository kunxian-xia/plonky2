extern crate core;

mod allocator;

use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use plonky2::field::extension::quadratic::QuadraticExtension;
use plonky2::field::extension::quartic::QuarticExtension;
use plonky2::field::extension::quintic::QuinticExtension;
use plonky2::field::goldilocks_field::GoldilocksField;
use plonky2::field::types::Field;
use plonky2_field::packable::Packable;
use plonky2_field::packed::PackedField;
use plonky2_field::types::Sample;
use tynm::type_name;

pub(crate) fn bench_field<F: Field>(c: &mut Criterion) {
    c.bench_function(&format!("mul-throughput<{}>", type_name::<F>()), |b| {
        b.iter_batched(
            || (F::rand(), F::rand(), F::rand(), F::rand()),
            |(mut x, mut y, mut z, mut w)| {
                for _ in 0..25 {
                    (x, y, z, w) = (x * y, y * z, z * w, w * x);
                }
                (x, y, z, w)
            },
            BatchSize::SmallInput,
        )
    });

    c.bench_function(&format!("mul-latency<{}>", type_name::<F>()), |b| {
        b.iter_batched(
            || F::rand(),
            |mut x| {
                for _ in 0..100 {
                    x = x * x;
                }
                x
            },
            BatchSize::SmallInput,
        )
    });

    c.bench_function(&format!("sqr-throughput<{}>", type_name::<F>()), |b| {
        b.iter_batched(
            || (F::rand(), F::rand(), F::rand(), F::rand()),
            |(mut x, mut y, mut z, mut w)| {
                for _ in 0..25 {
                    (x, y, z, w) = (x.square(), y.square(), z.square(), w.square());
                }
                (x, y, z, w)
            },
            BatchSize::SmallInput,
        )
    });

    c.bench_function(&format!("sqr-latency<{}>", type_name::<F>()), |b| {
        b.iter_batched(
            || F::rand(),
            |mut x| {
                for _ in 0..100 {
                    x = x.square();
                }
                x
            },
            BatchSize::SmallInput,
        )
    });

    c.bench_function(&format!("add-throughput<{}>", type_name::<F>()), |b| {
        b.iter_batched(
            || {
                (
                    F::rand(),
                    F::rand(),
                    F::rand(),
                    F::rand(),
                    F::rand(),
                    F::rand(),
                    F::rand(),
                    F::rand(),
                    F::rand(),
                    F::rand(),
                )
            },
            |(mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut h, mut i, mut j)| {
                for _ in 0..10 {
                    (a, b, c, d, e, f, g, h, i, j) = (
                        a + b,
                        b + c,
                        c + d,
                        d + e,
                        e + f,
                        f + g,
                        g + h,
                        h + i,
                        i + j,
                        j + a,
                    );
                }
                (a, b, c, d, e, f, g, h, i, j)
            },
            BatchSize::SmallInput,
        )
    });

    c.bench_function(&format!("add-latency<{}>", type_name::<F>()), |b| {
        b.iter_batched(
            || F::rand(),
            |mut x| {
                for _ in 0..100 {
                    x = x + x;
                }
                x
            },
            BatchSize::SmallInput,
        )
    });

    c.bench_function(&format!("try_inverse<{}>", type_name::<F>()), |b| {
        b.iter_batched(|| F::rand(), |x| x.try_inverse(), BatchSize::SmallInput)
    });

    c.bench_function(
        &format!("batch_multiplicative_inverse-tiny<{}>", type_name::<F>()),
        |b| {
            b.iter_batched(
                || (0..2).map(|_| F::rand()).collect::<Vec<_>>(),
                |x| F::batch_multiplicative_inverse(&x),
                BatchSize::SmallInput,
            )
        },
    );

    c.bench_function(
        &format!("batch_multiplicative_inverse-small<{}>", type_name::<F>()),
        |b| {
            b.iter_batched(
                || (0..4).map(|_| F::rand()).collect::<Vec<_>>(),
                |x| F::batch_multiplicative_inverse(&x),
                BatchSize::SmallInput,
            )
        },
    );

    c.bench_function(
        &format!("batch_multiplicative_inverse-medium<{}>", type_name::<F>()),
        |b| {
            b.iter_batched(
                || (0..16).map(|_| F::rand()).collect::<Vec<_>>(),
                |x| F::batch_multiplicative_inverse(&x),
                BatchSize::SmallInput,
            )
        },
    );

    c.bench_function(
        &format!("batch_multiplicative_inverse-large<{}>", type_name::<F>()),
        |b| {
            b.iter_batched(
                || (0..256).map(|_| F::rand()).collect::<Vec<_>>(),
                |x| F::batch_multiplicative_inverse(&x),
                BatchSize::LargeInput,
            )
        },
    );

    c.bench_function(
        &format!("batch_multiplicative_inverse-huge<{}>", type_name::<F>()),
        |b| {
            b.iter_batched(
                || (0..65536).map(|_| F::rand()).collect::<Vec<_>>(),
                |x| F::batch_multiplicative_inverse(&x),
                BatchSize::LargeInput,
            )
        },
    );
}

fn bench_packed_field<P: PackedField>(c: &mut Criterion) {
    c.bench_function(
        &format!("packed-mul-throughput-<{}>", type_name::<P>()),
        |b| {
            b.iter_batched(
                || vec![P::Scalar::rand_vec(P::WIDTH); 4],
                |mut elems| {
                    let mut elems = elems.iter_mut();
                    let a = elems.next().unwrap();
                    let b = elems.next().unwrap();
                    let c = elems.next().unwrap();
                    let d = elems.next().unwrap();

                    let pa = P::from_slice_mut(a);
                    let pb = P::from_slice_mut(b);
                    let pc = P::from_slice_mut(c);
                    let pd = P::from_slice_mut(d);

                    for _ in 0..25 {
                        pa.mul_assign(*pb);
                        pb.mul_assign(*pc);
                        pc.mul_assign(*pd);
                        pd.mul_assign(*pa);
                    }
                },
                BatchSize::SmallInput,
            )
        },
    );

    c.bench_function(
        &format!("packed-add-throughput-<{}>", type_name::<P>()),
        |b| {
            b.iter_batched(
                || vec![P::Scalar::rand_vec(P::WIDTH); 4],
                |mut elems| {
                    let mut elems = elems.iter_mut();
                    let a = elems.next().unwrap();
                    let b = elems.next().unwrap();
                    let c = elems.next().unwrap();
                    let d = elems.next().unwrap();

                    let pa = P::from_slice_mut(a);
                    let pb = P::from_slice_mut(b);
                    let pc = P::from_slice_mut(c);
                    let pd = P::from_slice_mut(d);

                    for _ in 0..25 {
                        pa.add_assign(*pb);
                        pb.add_assign(*pc);
                        pc.add_assign(*pd);
                        pd.add_assign(*pa);
                    }
                },
                BatchSize::SmallInput,
            )
        },
    );

    c.bench_function(&format!("packed-mul-latency<{}>", type_name::<P>()), |b| {
        b.iter_batched(
            || vec![P::Scalar::rand_vec(P::WIDTH); 1000],
            |x| {
                x.iter().fold(P::default(), |mut acc, x| {
                    let x = P::from_slice(x.as_slice());
                    acc.mul_assign(*x);

                    acc
                })
            },
            BatchSize::SmallInput,
        )
    });

    c.bench_function(&format!("packed-add-latency<{}>", type_name::<P>()), |b| {
        b.iter_batched(
            || vec![P::Scalar::rand_vec(P::WIDTH); 1000],
            |x| {
                x.iter().fold(P::default(), |mut acc, x| {
                    let x = P::from_slice(x.as_slice());
                    acc.add_assign(*x);

                    acc
                })
            },
            BatchSize::SmallInput,
        )
    });
}

fn criterion_benchmark(c: &mut Criterion) {
    bench_field::<GoldilocksField>(c);
    bench_field::<QuadraticExtension<GoldilocksField>>(c);
    bench_field::<QuarticExtension<GoldilocksField>>(c);
    bench_field::<QuinticExtension<GoldilocksField>>(c);
}

fn simd_benchmark(c: &mut Criterion) {
    bench_packed_field::<<GoldilocksField as Packable>::Packing>(c);
}

criterion_group!(benches, simd_benchmark, criterion_benchmark);
criterion_main!(benches);
