use super::*;
use halo2::halo2curves::bn256::Fr;
use {
    constants,
    gadgets::{apply_mds, pow},
};

fn permute(mut state: [Fr; 5]) -> [Fr; 5] {
    let fr = constants::full_rounds();
    let t = constants::sbox_exp();
    let rc = gadgets::round_constants();

    let mut round_constants_counter = 0;

    // First full rounds
    for round in 0..fr / 2 {
        // Round constants, nonlinear layer, matrix multiplication
        // 1. step Add Round Constants.
        for j in 0..t {
            state[j] = state[j] + rc[round_constants_counter];
            round_constants_counter += 1;
        }

        // 2. step applying S-boxes for the full round.
        for j in 0..t {
            state[j] = pow(state[j], 5)
        }

        // 3. step Mixlayer(matrix multiplication)
        state = apply_mds(state);
    }

    state
}
