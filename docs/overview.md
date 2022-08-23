# Terabethia

![Image](https://github.com/Psychedelic/terabethia/blob/master/docs/terabeta.png?raw=true "Design")

## Overview
Terabethia is a simple messaging protocol between Layer 1 (Ethereum) and Internet Computer. It’s using Starknet for cheap & efficient L2 → L1 message delivery.

If you want to build on top of Terabethia, you need to deploy a contract on Ethereum L1 as well as a canister on Internet Computer. These contracts need to be paired, which means your L1 contract needs to point to your L2 canister and vice versa.

### L1 → L2 flow
Any Ethereum contract can send message through Terabethia to IC by calling `Terabethia.sendMessage(uint256 canisterId, uint256[] payload)`. We store only the hash of the message (32 bytes) so that allows you to efficiently send even large payloads.

Once you submit the transaction to Ethereum, it’s picked up by our AWS Lambda. If the transaction is valid, we take uncompressed message data and forward the message to Terabethia on IC. The message is then automatically delivered by calling the `handler` method on your canister, then it’s up to your canister if you decide to consume the message. We provide an example below using `eth_proxy` .

Once the message is consumed from Terabethia (by calling `consume_message`), it’s gone and it can’t be consumed anymore. So it’s your responsibility to make sure your flow after `consume_message` is failure-proof. The [eth_proxy](https://github.com/Psychedelic/terabethia/blob/master/ic/w_eth/src/eth_proxy/src/lib.rs) example will provide two versions of how to consistently handle messages from our Terabethia canister. The first version will make the proxy an extension to a set token (i.e. DIP20, ERC721); allowing all the functionality to exist within one canister avoiding any inconsistencies with inter canister calls. The second version will build on top of the current version to maintain an internal log of received messages. Allowing manual (using heartbeat function) failure recovery to transactions between a proxy and the respective token.

This version of IC Terabethia focuses on internalizing and obfuscating data. Previously we didn’t have `nonce` on incoming messages. So with nonce now provided in the message we can maintain idempotency even with the same message. Similarly we maintain an internal index (`outgoing_nonce`) of outgoing messages to L1. Hashing the unique index with the message allows us to create a unique transaction list.

**⚠️ Warning: IC Principal is sensitive on padding. So if you’re converting Nat → Principal, you need to make sure to manually pad them (10 bytes for canister ids or 29 bytes for user principal ids).**

This statement is usually true (in Solidity, Node, etc):

`0x00000000003000F10101 === 0x3000F10101` 

However...

`Principal.fromHex('00000000003000F10101') ≠ Principal.fromHex(’3000F10101’)`

This fact might lead to serious mistakes, imagine:
```javascript
// lets say we want to mint wETH for this principal id
// it's a valid principal from a random identity
const principal = Principal.fromText('kyxzn-5aawk-7tlkc-pvrag-fioax-rhyre-nev4e-4lyc6-ifk4v-zrvlm-sae');

// 29b hex
const hex = principal.toHex(); // 00B2BF35A84FAC4062A1C0BC4F8891A4AF09C5E05E4155CAE6355B2402

// you can see 1st byte of principal is empty
// so it will be trimmed on L1
EthContract.deposit(hex, 1);

// when you receive the message on IC
// Nat will only have 28 bytes
// so without padding you'd get different Principal
```
We’ve created a Trait Principal::from_nat that handles this issue.

### L2 → L1 flow
It’s pretty much the same as L1→L2. Any IC canister can call `TerabethiaCanister.send_message(to: Principal, payload: Vec<Nat>)`. That store's outgoing message (hash only) on Terabethia canister. 

We pull these messages by our AWS Lambda every minute. Every message is sent to L1 through our Starknet Cairo contract. The main difference is L1 messages **are not** automatically triggered (because of gas costs), so once the message is accepted on L1, a user needs to manually consume the message. 

As an example, you can check our [EthProxy.sol](https://github.com/Psychedelic/terabethia/blob/master/eth_bridge/eth/contracts/EthProxy.sol), where users can invoke `withdraw(uint256 amount)`. **You can only withdraw the exact amount you burnt on L2.**

## How long does it take to be accepted on L1?

Usually, within an hour, it depends on Starknet. There will be a way how to speed this up by providing payment.

### AWS Security Guarantees

#### Internet Computer Signatures

Sending messages from L1 → L2 is secured by `secp256k1` key stored in AWS KSM, which means private key never leaves HSM (Hardware Security Module). Yes, even developers do not have access to it. We only know the public key (principal id), which is needed for whitelisting on the IC. 

Signing action (`ksm:sign`) is **disabled for everyone**, except the lambda roles which are responsible for polling/sending messages (to the IC).

#### Starknet Signatures

*Starknet does not support `secp256k1` signatures, we are working on it in branch `feat/secp256k1-cairo`.* 

Meanwhile, we have another AWS KSM key that’s used for decrypting Starknet private key. But how it was encrypted? For this purpose we have a simple bootstrap Lambda, it creates random key pair for Starknet and it only exposes stark **address** and **encrypted** **private key** in base64 format. Stark address is used when we deploy Cairo contracts (whitelisting operator) and an encrypted private key is set as env variable on Lambda that sends messages to Starknet.

Encrypting via `ksm:encrypt` is only allowed to bootstrap the Lambda role. Decrypting via `ksm:decrypt` is only allowed to a Lambda that sends messages to Starknet.

---

### Magic Bridge
With the magic bridge we bring a seamless interface to bridge over all your ERC20 tokens from Ethereum to the IC. Similar to our approach to the WETH Proxy on IC, the magic bridge is composed of mirror contracts on both chains for each token type, ERC20, and ERC721. The only difference is our magic factory on IC, which we will cover in the next section.

We start on L1 with a new contract ERC20Brdige responsible for passing ERC20 metadata in a message, the data in the message includes:

```Solidity
  token        // address of the ERC20
  user         // owner of the tokens
  amount       // the amount we send over
  tokenName    // the name of the ERC20
  tokenSymbol  // the symbol of the ERC20
  decimalds    // the decimals of the ERC20
```
The message then gets sent to our Terabethia canister on IC to be processed, exactly like any other message. An end user would only need to provide the token address, amount, and their address. Everything else is pulled from chain. 

#### ETH → IC
Once the message lands on the Terabethia IC canister we send the message to the DIP20Bridge which handles the message and verifies the origin Ethereum contract. Additionally, using our MagicFactory the bridge checks if the origin ETH address has a corresponding pair canister. This  is the mirror address which will handle all the transactions for this specific origin ETH address. If one such pair canister does not exist it will create one using the  metadata in the message and assign 1-1 to the origin address. From there we can mint the incoming transaction in the corresponding canister eth address -> canister_id for user. 

#### IC → ETH
The burn flow is very similar to our previous release with the WETHProxy, where you approve the DIP20Proxy for the amount you want to burn back to L1 and then call the burn()method on the DIP20Bridge. So how is the burn handled? Firstly, after the approval is initiated by the end user, and the burn call is initiated, we make a transfer to our bridge canister for the same amount and credit the end user that same amount on our DIP20Bridge. Lastly, we send_message to the Terabethia IC canister. The local user credit becomes useful in case any of the other calls inside the function fail. With that, we don’t have to worry about any atomicity issues with these calls. Also, because we have control over the entire pipeline, we make the process fault tolerant. 

---

## Instructions

### Install

#### Core

ETH:
```shell
cd core/eth/
yarn
```

Starknet: [env setup](https://www.cairo-lang.org/docs/quickstart.html#quickstart)
```shell
cd core/starknet

# activate your environment
source ~/cairo_venv/bin/activate
```

#### ETH Bridge
```shell
cd eth_bridge/eth/
yarn
```
#### Magic Bridge
```shell
cd magic_bridge/eth/
yarn
```


### Deploy

#### Core
  - Starknet: [Instructions](https://github.com/Psychedelic/terabethia/tree/update_starknet_version/core/starknet/README.md)
    - You also use the [script](https://github.com/Psychedelic/terabethia/blob/master/core/starknet/TESTNET.sh) to deploy everything. 
    - This will deploy the terabethia contract, its proxy and the account operators.

  - Eth: [Instructions](https://github.com/Psychedelic/terabethia/blob/update_starknet_version/core/eth/README.md) 
    - Modify the starknet contract consts in the ethereum contract and deploy script located here:
      - [deploy.ts#L3](https://github.com/Psychedelic/terabethia/blob/master/core/eth/scripts/deploy.ts#L3)
      - [Terabethia.sol#L15](https://github.com/Psychedelic/terabethia/blob/master/core/eth/contracts/Terabethia.sol#L15)

#### ETH Bridge
  - Eth: [Instructions](https://github.com/Psychedelic/terabethia/blob/master/eth_bridge/eth/README.md) 
    - Modify the starknet contract consts in the deploy script located here:
      - [deploy.ts#L3](https://github.com/Psychedelic/terabethia/blob/master/eth_bridge/eth/scripts/deploy.ts)

#### Magic Bridge
  - Eth: [Instructions](https://github.com/Psychedelic/terabethia/blob/master/magic_bridge/eth/README.md) 
    - Modify the starknet contract consts in the deploy script located here:
      - [deploy.ts#L3](https://github.com/Psychedelic/terabethia/blob/master/magic_bridge/eth/scripts/deploy.ts)

### Tests

#### Core

ETH:
```shell
cd core/eth/
npx hardhat test
```

Starknet:
```
cd core/starknet
pytest cairo/terabethia_test.py
```

#### ETH Bridge
```shell
cd eth_bridge/eth/
npx hardhat test
```
#### Magic Bridge
```shell
cd magic_bridge/eth/
npx hardhat test
```
