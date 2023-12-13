# fft for polynomial of degrees from 19 to 23
# lde onto coset for polynomial of degrees from 20 to 23 and rate_bits from 1 to 3.
cargo bench --package plonky2 --bench ffts

# poseidon / keccak permutation
cargo bench --package plonky2 --bench hashing

# merkle tree root
cargo bench --package plonky2 --bench merkle