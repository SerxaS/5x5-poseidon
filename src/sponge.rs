use ark_bn254::Fr;

use crate::hasher::permute;
use ark_ff::Zero;

/// Absorb the data and split it chunk of size 5.
pub fn load_state(chunk: &[Fr]) -> [Fr; 5] {
    assert!(chunk.len() <= 5);
    let mut fixed_chunk = [Fr::zero(); 5];
    fixed_chunk[..chunk.len()].copy_from_slice(chunk);
    fixed_chunk
}

/// Squeeze the data out by permuting until no more chunks are left.
pub fn squeeze(mut inputs: Vec<Fr>) -> Fr {
    if inputs.is_empty() {
        inputs.push(Fr::zero());
    }

    let mut state = [Fr::zero(); 5];

    for chunk in inputs.chunks(5) {
        let mut temp_inputs = [Fr::zero(); 5];

        // Absorb
        let loaded_state = load_state(chunk);

        for i in 0..5 {
            temp_inputs[i] = loaded_state[i] + state[i];
        }

        state = permute::permute(temp_inputs);
    }

    // Returns the first element of state.
    state[0]
}
