use super::*;

use ark_bn254::Fr;

use {
    constants,
    gadgets::{apply_mds, pow},
};

/// Mixing rounds with half-full S-box layers and
/// rounds with partial S-box layers.
/// More detailed explanation for
/// https://eprint.iacr.org/2019/458
pub fn permute(inputs: [Fr; 5]) -> [Fr; 5] {
    let fr = constants::full_rounds();
    let fp = constants::partial_rounds();
    let rc = gadgets::round_constants();

    let mut state = inputs;
    let mut round_constants_counter = 0;

    // First full rounds.
    for round in 0..fr / 2 {
        // Round constants, nonlinear layer, matrix multiplication.
        // 1. step Add Round Constants.
        for i in 0..5 {
            state[i] = state[i] + rc[round_constants_counter];
            round_constants_counter += 1;
        }

        // 2. step applying S-boxes for the full round.
        for i in 0..5 {
            state[i] = pow(state[i], 5)
        }

        // 3. step Mixlayer(matrix multiplication).
        state = apply_mds(state);
    }

    // Middle partial rounds.
    for round in 0..fp {
        // Round constants, nonlinear layer, matrix multiplication.
        // 1. step Add Round Constants.
        for i in 0..5 {
            state[i] = state[i] + rc[round_constants_counter];
            round_constants_counter += 1;
        }

        // 2. step applying S-boxes for the partial round.
        state[0] = pow(state[0], 5);

        // 3. step Mixlayer(matrix multiplication).
        state = apply_mds(state);
    }

    // Last full rounds.
    for round in 0..fr / 2 {
        // Round constants, nonlinear layer, matrix multiplication.
        // 1. step Add Round Constants.
        for i in 0..5 {
            state[i] = state[i] + rc[round_constants_counter];
            round_constants_counter += 1;
        }

        // 2. step applying S-boxes for the full round.
        for i in 0..5 {
            state[i] = pow(state[i], 5)
        }

        // 3. step Mixlayer(matrix multiplication).
        state = apply_mds(state);
    }

    state
}
