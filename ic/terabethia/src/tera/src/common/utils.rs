use candid::Nat;
use sha3::{Digest, Keccak256};

use super::types::{IncomingMessageHashParams, Message, OutgoingMessageHashParams};

pub trait Keccak256HashFn<T> {
    fn calculate_hash(&self, params: T) -> String;
}

impl Keccak256HashFn<IncomingMessageHashParams> for Message {
    fn calculate_hash(&self, params: IncomingMessageHashParams) -> String {
        let mut data = vec![
            params.from,
            params.to,
            Nat::from(params.nonce),
            Nat::from(params.payload.len()),
        ];
        data.extend(params.payload);

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
}

impl Keccak256HashFn<OutgoingMessageHashParams> for Message {
    fn calculate_hash(&self, params: OutgoingMessageHashParams) -> String {
        let mut data = vec![params.from, params.to, Nat::from(params.payload.len())];
        data.extend(params.payload);

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
}
