use std::fmt;

use ic_kit::candid::Nat;
use sha3::{Digest, Keccak256};

use super::types::{
    FactoryError, IncomingMessageHashParams, Message, MessageHash, OutgoingMessageHashParams,
};

impl fmt::Display for FactoryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FactoryError::CreateCanisterError => write!(f, "CreateCanisterError"),
            FactoryError::CanisterStatusNotAvailableError => {
                write!(f, "CanisterStatusNotAvailableError")
            }
            FactoryError::EncodeError => write!(f, "EncodeError"),
            FactoryError::CodeAlreadyInstalled => write!(f, "CodeAlreadyInstalled"),
            FactoryError::InstallCodeError => write!(f, "InstallCodeError"),
        }
    }
}

pub trait Keccak256HashFn<T> {
    fn calculate_hash(&self, params: T) -> MessageHash;
}

impl Keccak256HashFn<IncomingMessageHashParams> for Message {
    fn calculate_hash(&self, params: IncomingMessageHashParams) -> MessageHash {
        let mut data = vec![
            params.from,
            params.to,
            params.nonce,
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
                [&f[..l], slice].concat()
            })
            .collect();

        let concated = data_encoded.concat().to_vec();
        let mut hasher = Keccak256::new();

        hasher.update(concated);

        let result = hasher.finalize();

        hex::encode(result)
    }
}

impl Keccak256HashFn<OutgoingMessageHashParams> for Message {
    fn calculate_hash(&self, params: OutgoingMessageHashParams) -> MessageHash {
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
                [&f[..l], slice].concat()
            })
            .collect();

        let concated = data_encoded.concat().to_vec();
        let mut hasher = Keccak256::new();

        hasher.update(concated);

        let result = hasher.finalize();

        hex::encode(result)
    }
}
