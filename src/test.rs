#[cfg(test)]
mod test {
    use crate::hasher::{gadgets::hex_to_field, permute};
    use halo2::halo2curves::bn256::Fr;

    #[test]
    fn test_poseidon_5x5() {
        // Testing 5x5 input.
        let inputs: [Fr; 5] = [
            "0x0000000000000000000000000000000000000000000000000000000000000000",
            "0x0000000000000000000000000000000000000000000000000000000000000001",
            "0x0000000000000000000000000000000000000000000000000000000000000002",
            "0x0000000000000000000000000000000000000000000000000000000000000003",
            "0x0000000000000000000000000000000000000000000000000000000000000004",
        ]
        .map(|n| hex_to_field(n));

        let outputs: [Fr; 5] = [
            "0x299c867db6c1fdd79dcefa40e4510b9837e60ebb1ce0663dbaa525df65250465",
            "0x1148aaef609aa338b27dafd89bb98862d8bb2b429aceac47d86206154ffe053d",
            "0x24febb87fed7462e23f6665ff9a0111f4044c38ee1672c1ac6b0637d34f24907",
            "0x0eb08f6d809668a981c186beaf6110060707059576406b248e5d9cf6e78b3d3e",
            "0x07748bc6877c9b82c8b98666ee9d0626ec7f5be4205f79ee8528ef1c4a376fc7",
        ]
        .map(|n| hex_to_field(n));

        let out = permute::permute(inputs);

        assert_eq!(out, outputs);
    }
}
