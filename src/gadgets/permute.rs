use crate::gadgets::constants;
use halo2::halo2curves::bn256::Fr;

fn permute(state_words: [Fr; 5]) -> [Fr; 5] {
    let fr = constants::full_rounds();
    let t = constants::sbox_exp();

    let round_constants_counter = 0;

    // First full rounds
    for i in 0..fr / 2 {
        // Round constants, nonlinear layer, matrix multiplication
        for j in 0..t {
            state_words[j] = state_words[j] + round_constants_field[round_constants_counter];
        }
    }

    inputs
}
