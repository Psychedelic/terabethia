use candid::Nat;
use sha3::{Digest, Keccak256};

pub fn calculate_hash(from: Nat, to: Nat, payload: Vec<Nat>) -> String {
    let mut data = vec![from, to, Nat::from(payload.len())];
    data.extend(payload);

    let data_encoded: Vec<Vec<u8>> = data
        .clone()
        .into_iter()
        .map(|x| {
            // take a slice of 32
            let f = [0u8; 32];
            let slice = &x.0.to_bytes_be()[..];
            // calculate zero values padding
            let l = 32 - slice.len();
            [&f[..l], &slice].concat()
        })
        .collect();

    let concated = data_encoded.concat().to_vec();

    let mut hasher = Keccak256::new();

    hasher.update(concated);

    let result = hasher.finalize();

    hex::encode(result.to_vec())
}
