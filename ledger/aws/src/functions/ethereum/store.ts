import 'source-map-support/register';

import { BigNumber, ethers } from 'ethers';
import { Terabethia, KMSIdentity } from '@libs/dfinity';
import { Secp256k1PublicKey } from '@dfinity/identity';
import { Principal } from '@dfinity/principal';
import TerabethiaAbi from '@libs/eth/abi/Terabethia.json';
import { BlockNativePayload } from '@libs/blocknative';
import EthereumDatabase from '@libs/dynamo/ethereum';
import { sqsHandler, requireEnv } from '@libs/utils';
import bluebird from 'bluebird';
import BN from 'bn.js';
import {
  KMSClient,
} from '@aws-sdk/client-kms';

const envs = requireEnv(['ETHEREUM_TABLE_NAME',
  'ETHEREUM_PROVIDER_URL',
  'CANISTER_ID',
  'QUEUE_URL',
  'ETHEREUM_CONTRACT',
  'KMS_KEY_ID',
  'KMS_PUBLIC_KEY',
]);

// Terabethia IC with KMS
const kms = new KMSClient({});
const publicKey = Secp256k1PublicKey.fromRaw(Buffer.from(envs.KMS_PUBLIC_KEY, 'base64'));
const identity = new KMSIdentity(publicKey, kms, envs.KMS_KEY_ID);

// const proxy = new Proxy(envs.ETH_PROXY_ID, identity)

// const terabethia = new Terabethia(envs.CANISTER_ID, identity);
// Terabethia ETH
const db = new EthereumDatabase(envs.ETHEREUM_TABLE_NAME);
const provider = new ethers.providers.StaticJsonRpcProvider(envs.ETHEREUM_PROVIDER_URL);
const ethContract = new ethers.Contract(envs.ETHEREUM_CONTRACT, TerabethiaAbi, provider);


const handleWithdraw = async (message: BlockNativePayload) => {
  const { hash } = message;

  // we check the message exist
  console.log(`tx hash: ${hash}`);
  if (await db.hasTransaction(hash)) {
    console.log('tx was already processed');
    return;
  }

  // we get the tx receipt 
  await provider.ready;
  const receipt = await provider.getTransactionReceipt(hash);
  if (!receipt) {
    throw new Error('receipt is not available yet');
  }

  // get the transaction logs
  let logs = [];
  try {
    logs = receipt.logs.map((log) => {
      try {
        return ethContract.interface.parseLog(log);
      } catch (e) {
        return null;
      }
    }).filter((log) => log && log.args && log.args.from_address);
  } catch (e) {
    console.log(e);
    console.log('error during parsing logs, exiting');
    return;
  }

  // we discard empty logs
  if (!logs.length) {
    // ignore this tx
    console.log('transaction without logs, exiting');
    return;
  }



}

// const handleL1Message = async (message: BlockNativePayload) => {
//   const { hash } = message;

//   console.log(`tx hash: ${hash}`);

//   const hasTx = await db.hasTransaction(hash);

//   // we do not process transaction when it's already processed
//   if (hasTx) {
//     console.log('tx was already processed');
//     return;
//   }

//   await provider.ready;

//   const receipt = await provider.getTransactionReceipt(hash);

//   if (!receipt) {
//     throw new Error('receipt is not available yet');
//   }

//   let logs = [];
//   try {
//     logs = receipt.logs.map((log) => {
//       try {
//         return ethContract.interface.parseLog(log);
//       } catch (e) {
//         return null;
//       }
//     }).filter((log) => log && log.args && log.args.from_address);
//   } catch (e) {
//     console.log(e);
//     console.log('error during parsing logs, exiting');
//     return;
//   }
//   if (!logs.length) {
//     // ignore this tx
//     console.log('transaction without logs, exiting');
//     return;
//   }

//   // we need to loop through the logs, because 1 transaction can emit multiple messages
//   await bluebird.each(logs, async (log) => {
//     const {
//       from_address: fromAddress, to_address: toAddress, nonce, payload,
//     } = log.args;

//     // recompute messageHash
//     const messageHash = ethers.utils.solidityKeccak256(
//       ['uint256', 'uint256', 'uint256', 'uint256', 'uint256[]'],
//       [fromAddress, toAddress, nonce, payload.length, payload],
//     );

//     console.log({
//       fromAddress, toAddress, nonce, payloadLength: payload.length, payload,
//     });

//     const hasMessageHash = await db.hasMessageHash(messageHash);

//     if (hasMessageHash) {
//       // already processed
//       console.log('this message was already processed');
//       return;
//     }

//     // check if the hash actually exists on L1
//     const number = await ethContract.messages(messageHash);

//     if (number.isZero()) {
//       throw new Error(`Message hash ${messageHash} is not valid.`);
//     }

//     // fromAddress is hex string prefixed with 0x
//     const fromAddresPid = Principal.fromHex(fromAddress.substr(2));

//     // toAddress is ethers BigNumber, we convert it to uint8array (BigEndian)
//     const arr = new BN(toAddress.toBigInt()).toArray();

//     // we also need to handle 10 bytes padding for canister ids
//     const paddedArr = new Uint8Array(Array(10 - arr.length).fill(0).concat(arr));
//     const toAddressPid = Principal.fromUint8Array(paddedArr);

//     console.log({
//       fromPrincipal: fromAddresPid.toText(),
//       toPrincipal: toAddressPid.toText(),
//       nonce,
//       payload,
//     });

//     const payloadBigInt = payload.map((p: BigNumber) => p.toBigInt());

//     await terabethia.storeMessage(fromAddresPid, toAddressPid, nonce.toBigInt(), payloadBigInt);
//     await db.storeMessageHash(messageHash);
//   });

//   await db.storeTransaction(hash);
// };

export const main = sqsHandler<BlockNativePayload>(handleWithdraw, envs.QUEUE_URL, undefined, 1);
