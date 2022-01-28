import 'source-map-support/register';

import { ethers } from 'ethers';
import { Terabethia } from '@libs/dfinity';
import { Principal } from '@dfinity/principal';
import TerabethiaAbi from '@libs/eth/abi/Terabethia.json';
import { BlockNativePayload } from '@libs/blocknative';
import EthereumDatabase from '@libs/dynamo/ethereum';
import { sqsHandler } from '@libs/utils';
import bluebird from 'bluebird';
import BN from 'bn.js';

const {
  ETHEREUM_TABLE_NAME,
  ETHEREUM_PROVIDER_URL,
  IC_PRIVATE_KEY,
  IC_CANISTER_ID,
  QUEUE_URL,
  ETHEREUM_CONTRACT,
} = process.env;

if (!ETHEREUM_TABLE_NAME) {
  throw new Error('ETHEREUM_TABLE_NAME must be set');
}

if (!ETHEREUM_PROVIDER_URL) {
  throw new Error('ETHEREUM_PROVIDER_URL must be set');
}

if (!ETHEREUM_CONTRACT) {
  throw new Error('ETHEREUM_CONTRACT must be set');
}

if (!IC_PRIVATE_KEY) {
  throw new Error('IC_PRIVATE_KEY must be set');
}

if (!IC_CANISTER_ID) {
  throw new Error('IC_CANISTER_ID must be set');
}

if (!QUEUE_URL) {
  throw new Error('IC_CANISTER_ID must be set');
}

const db = new EthereumDatabase(ETHEREUM_TABLE_NAME);
const provider = new ethers.providers.StaticJsonRpcProvider(ETHEREUM_PROVIDER_URL);
const terabethia = new Terabethia(IC_CANISTER_ID, IC_PRIVATE_KEY);
const ethContract = new ethers.Contract(ETHEREUM_CONTRACT, TerabethiaAbi, provider);

const handleL1Message = async (message: BlockNativePayload) => {
  const { hash } = message;
  console.log(`hash: ${hash}`);

  const hasTx = await db.hasTransaction(hash);

  // we do not process transaction when it's already processed
  if (hasTx) {
    return;
  }

  await provider.ready;

  const receipt = await provider.getTransactionReceipt(hash);
  const logs = receipt.logs.map((log) => ethContract.interface.parseLog(log)).filter((log) => log.args && log.args.from_address);

  if (!logs.length) {
    throw new Error('Transaction did not emit any logs.');
  }

  // we need to loop through the logs, because 1 transaction can emit multiple messages
  await bluebird.each(logs, async (log) => {
    const {
      from_address: fromAddress, to_address: toAddress, nonce, payload,
    } = log.args;

    // recompute messageHash
    const messageHash = ethers.utils.solidityKeccak256(
      ['uint256', 'uint256', 'uint256', 'uint256', 'uint256[]'],
      [fromAddress, toAddress, nonce, payload.length, payload],
    );

    const hasMessageHash = await db.hasMessageHash(messageHash);

    if (hasMessageHash) {
      // already processed
      return;
    }

    // check if the hash actually exists on L1
    const number = await ethContract.messages(messageHash);

    if (new BN(number).isZero()) {
      throw new Error(`Message hash ${messageHash} is not valid.`);
    }

    // fromAddress is hex string prefixed with 0x
    const fromAddresPid = Principal.fromHex(fromAddress.substring(2));

    // toAddress is big number
    const toAddressPid = Principal.fromHex(new BN(toAddress).toString('hex'));

    await terabethia.storeMessage(fromAddresPid, toAddressPid, nonce, payload);
    await db.storeMessageHash(messageHash);
  });

  await db.storeTransaction(hash);
};

export const main = sqsHandler<BlockNativePayload>(handleL1Message, QUEUE_URL, undefined, 1);
