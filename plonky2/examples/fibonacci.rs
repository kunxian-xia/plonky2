use anyhow::Result;
use log::Level;
use plonky2::field::types::Field;
use plonky2::iop::witness::{PartialWitness, WitnessWrite};
use plonky2::plonk::circuit_builder::CircuitBuilder;
use plonky2::plonk::circuit_data::CircuitConfig;
use plonky2::plonk::config::{GenericConfig, PoseidonGoldilocksConfig};
use plonky2::plonk::prover::prove;
use plonky2::util::timing::TimingTree;

/// An example of using Plonky2 to prove a statement of the form
/// "I know the 100th element of the Fibonacci sequence, starting with constants a and b."
/// When a == 0 and b == 1, this is proving knowledge of the 100th (standard) Fibonacci number.
fn main() -> Result<()> {
    const D: usize = 2;
    type C = PoseidonGoldilocksConfig;
    type F = <C as GenericConfig<D>>::F;

    env_logger::init();
    let config = CircuitConfig::standard_recursion_config();
    let mut builder = CircuitBuilder::<F, D>::new(config);

    // 135 wires (55 wires no copy)
    // 80 routable wires (80 columns in permutation)
    // 1 arithmetic gate: 4 wires,
    // 1 row 20 arithmetic gates
    // 100 addition => 100 arithmetic => 5 rows.

    // 1 custom gate: l0' = l1, l1' = l0 + l1.

    // The arithmetic circuit.
    let initial_a = builder.add_virtual_target();
    let initial_b = builder.add_virtual_target();
    let mut prev_target = initial_a;
    let mut cur_target = initial_b;
    for _ in 0..99 {
        let temp = builder.add(prev_target, cur_target);
        prev_target = cur_target;
        cur_target = temp;
    }

    // Public inputs are the two initial values (provided below) and the result (which is generated).
    builder.register_public_input(initial_a);
    builder.register_public_input(initial_b);
    builder.register_public_input(cur_target);

    // Provide initial values.
    let mut pw = PartialWitness::new();
    pw.set_target(initial_a, F::ZERO);
    pw.set_target(initial_b, F::ONE);

    builder.print_gate_counts(0);
    let data = builder.build::<C>();

    let mut timing = TimingTree::new("prove", Level::Debug);
    let proof = prove::<F, C, D>(&data.prover_only, &data.common, pw, &mut timing)?;
    timing.print();

    println!(
        "100th Fibonacci number mod |F| (starting with {}, {}) is: {}",
        proof.public_inputs[0], proof.public_inputs[1], proof.public_inputs[2]
    );

    data.verify(proof)
}
