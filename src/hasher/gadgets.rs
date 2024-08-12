use super::*;
use halo2::halo2curves::{bn256::Fr, ff::FromUniformBytes};

/// Calculates exponent of a given field element.
pub fn pow(base: Fr, exp: usize) -> Fr {
    let mut mul = Fr::one();

    for _ in 0..exp {
        mul *= base
    }
    mul
}

/// Returns congruent field element for the given hex string.
pub fn hex_to_field(item: &str) -> Fr {
    let item = &item[2..];
    let mut bytes = hex::decode(item).expect("Invalid parameters!");
    bytes.reverse();
    let mut temp_bytes = [0; 64];
    temp_bytes[..bytes.len()].copy_from_slice(&bytes[..]);
    Fr::from_uniform_bytes(&temp_bytes)
}

/// Returns the round constants to be used in the permutation without the prefix.
pub fn round_constants() -> Vec<Fr> {
    let round_constants_raw = constants::round_constants_raw();
    let round_constants: Vec<Fr> = round_constants_raw
        .iter()
        .map(|item| hex_to_field(item))
        .collect();
    round_constants
}

/// Returns the mds matrix elements without prefix.
pub fn mds() -> [[Fr; 5]; 5] {
    let mds_raw = constants::mds_raw();
    mds_raw.map(|row| row.map(|item| hex_to_field(item)))
}

/// Computes MDS matrix for MixLayer operation.
pub fn apply_mds(state: [Fr; 5]) -> [Fr; 5] {
    let mds = mds();
    let mut new_state = [Fr::zero(); 5];

    for i in 0..5 {
        for j in 0..5 {
            new_state[i] += state[j] * mds[i][j]
        }
    }

    new_state
}
