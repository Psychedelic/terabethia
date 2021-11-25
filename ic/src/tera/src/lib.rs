use ethabi::encode;
use ethabi::ethereum_types::U256;
use ic_cdk::export::Principal;
use ic_cdk::{api, caller};
use ic_cdk_macros::update;
use sha3::{Digest, Keccak256};

fn calculate_hash(from: Vec<u8>, to: Vec<u8>, payload: Vec<Vec<u8>>) -> Vec<u8> {
    let receiver = ethabi::Token::FixedBytes(to);
    let sender = ethabi::Token::FixedBytes(from);
    let payload_len = ethabi::Token::Uint(U256::from(payload.len()));

    // we map payload to FixedBytes
    // becase on L1 these are left padded to 32b
    let payload_padded: Vec<ethabi::Token> = payload
        .into_iter()
        .map(|x| ethabi::Token::FixedBytes(x.clone()))
        .collect();

    let payload_slice = &payload_padded[..];
    let tokens_slice = &[&[sender, receiver, payload_len], payload_slice].concat()[..];

    let encoded = encode(tokens_slice);

    let mut hasher = Keccak256::new();

    hasher.update(encoded);

    let result = hasher.finalize();

    return result.to_vec();
}

/**
 * This method is called by AWS Lambda. Purpose of this method is to
 * trigger generic handler method which should be implemented by the "to" canister.
 *
 * @todo: add controller/operator guard
 * @todo: once Eth integration is available on the IC, we should not store messages here.
 * Instead we'll check state against Eth contract directly.
 * */
#[update(name = "receiveMessageFromL1")]
pub async fn receive(from: Vec<u8>, to: Principal, payload: Vec<Vec<u8>>) -> Result<bool, String> {
    if api::id() == caller() {
        return Err("Attempted to call handler on self. This is not allowed..".to_string());
    }

    let msgHash = calculate_hash(from, to.clone().as_slice().to_vec(), payload);

    // encode();
    // encode()
    // @todo: decode payload to vec nat
    // calculate message hash
    // store message

    // @todo: encode args_raw={to, payload} to be Vec<u8>

    match api::call::call_raw(to.clone(), "handler", [].to_vec(), 0).await {
        Ok(x) => Ok(true),
        Err((code, msg)) => Err(format!(
            "An error happened during the call: {}: {}",
            code as u8, msg
        )),
    }
}

// this method should be called by canisters only
#[update(name = "consumeMessageFromL1")]
pub async fn consume(contract: String, payload: Vec<u8>) -> Result<bool, String> {
    let caller = api::id();

    // @todo: decode payload to vec nat
    // calculate message hash
    // store message hash

    unimplemented!()
}

// this method should be called by canisters only
#[update(name = "sendMessageToL1")]
pub async fn send(contract: String, payload: Vec<u8>) -> Result<bool, String> {
    let caller = api::id();

    // @todo: decode payload to vec nat
    // calculate message hash
    // store message hash

    unimplemented!()
}

#[cfg(test)]
mod tests {
    use crate::calculate_hash;

    #[test]
    fn message_hash() {
        let from = hex::decode("6d6e6932637a71616161616161616471616c3671636169000000000000000000")
            .unwrap();

        let to = hex::decode("000000000000000000000000d2f69519458c157a14c5caf4ed991904870af834")
            .unwrap();
        let payload = [
            hex::decode("0000000000000000000000000000000000000000000000000000000000000000")
                .unwrap(),
            hex::decode("000000000000000000000000f39fd6e51aad88f6f4ce6ab8827279cfffb92266")
                .unwrap(),
            hex::decode("000000000000000000000000000000000000000000000000016345785d8a0000")
                .unwrap(), // 0.1 eth value
        ]
        .to_vec();

        let msgHash = calculate_hash(from, to, payload);
        let msgHashHex = hex::encode(msgHash.clone());

        println!("msg hash hex {} arguments", msgHashHex);

        // [128, 62, 240, 110, 171, 68, 239, 5, 218, 94, 164, 227, 190, 40, 195, 19, 138, 53, 191, 94, 129, 225, 113, 205, 28, 247, 125, 81, 119, 34, 39, 138]
        let msgHashExpected =
            hex::decode("a0651ef3ef5db8ae814a37abf8e63cbe88d0194789edc362951825bd4b2c5c55")
                .unwrap();

        assert_eq!(msgHash, msgHashExpected);
    }
}

// const withdrawPayload = [
//     '0x0000000000000000000000000000000000000000000000000000000000000000',
//     numStringToBytes32(Buffer.from('f39fd6e51aad88f6f4ce6ab8827279cfffb92266', 'hex')),
//     numStringToBytes32(ethValue2.toString()), // should be 0x000000000000000000000000000000000000000000000000016345785d8a0000
//   ];

//   expect(withdrawPayload[1]).equal('0x000000000000000000000000f39fd6e51aad88f6f4ce6ab8827279cfffb92266')
//   expect(withdrawPayload[2]).equal('0x000000000000000000000000000000000000000000000000016345785d8a0000')

//   const withdrawMessageHash = soliditySha3(
//     "0x6d6e6932637a71616161616161616471616c3671636169000000000000000000",
//     ethProxy.address,
//     withdrawPayload.length,
//     { t: 'bytes32', v: withdrawPayload }
//   );

//   // 0xefb80e98c9f7ac2ad55b3e4f5bb2d3a15fe8c187925eba2ffc721f74d1982c52
//   expect(withdrawMessageHash).equals('0xff76cffb15cc5fbb35ba768c1aa7a821ccd5e4901c4ff733ea941747a2a52413');
