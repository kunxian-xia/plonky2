mod allocator;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use plonky2::field::goldilocks_field::GoldilocksField;
use plonky2::hash::hash_types::RichField;
use plonky2::hash::keccak::KeccakHash;
use plonky2::hash::merkle_tree::MerkleTree;
use plonky2::hash::poseidon::PoseidonHash;
use plonky2::plonk::config::Hasher;
use tynm::type_name;

// const ELEMS_PER_LEAF: usize = 135;

pub(crate) fn bench_merkle_tree<F: RichField, H: Hasher<F>>(
    c: &mut Criterion,
    elems_per_leaf: usize,
) {
    let mut group = c.benchmark_group(&format!(
        "merkle-tree<{}, {}>",
        // type_name::<F>(),
        type_name::<H>(),
        elems_per_leaf,
    ));
    group.sample_size(10);

    for size_log in [13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25] {
        let size = 1 << size_log;
        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, _| {
            let leaves = vec![F::rand_vec(elems_per_leaf); size];
            b.iter(|| MerkleTree::<F, H>::new(leaves.clone(), 0));
        });
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    for elems_per_leaf in [135, 270, 405, 540, 675] {
        bench_merkle_tree::<GoldilocksField, PoseidonHash>(c, elems_per_leaf);
    }
    for elems_per_leaf in [135, 270, 405, 540, 675] {
        bench_merkle_tree::<GoldilocksField, KeccakHash<25>>(c, elems_per_leaf);
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
