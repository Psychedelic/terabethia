import 'source-map-support/register';

import { ethers } from 'ethers';
import { EthProxy, KMSIdentity } from '@/libs/dfinity';
import { Principal } from '@dfinity/principal';
import { BlockNativePayload } from '@/libs/blocknative';
import { sqsHandler } from '@/libs/utils';

import payload from './mock.json';
import { KMSClient } from '@aws-sdk/client-kms';
import { Secp256k1PublicKey } from '@dfinity/identity';

const envs = requireEnv([
  'ETHEREUM_PROVIDER_URL',
  'CANISTER_ID',
  'QUEUE_URL',
  'ETHEREUM_CONTRACT',
  'KMS_KEY_ID',
  'KMS_PUBLIC_KEY',
]);

// EthProxy ETH
const provider = new ethers.providers.StaticJsonRpcProvider(envs.ETHEREUM_PROVIDER_URL);

// EthProxy IC with KMS
const kms = new KMSClient({});
const publicKey = Secp256k1PublicKey.fromRaw(Buffer.from(envs.KMS_PUBLIC_KEY, 'base64'));
const identity = new KMSIdentity(publicKey, kms, envs.KMS_KEY_ID);
const eth_proxy = new EthProxy(envs.ETH_PROXY_CANISTER_ID, identity);

export const handleWithdraw = async (message: BlockNativePayload) => {
  if (message.status !== 'confirmed') {
    throw new Error('transaction is not confirmed yet');
  }

  const { hash } = message;
  const { params: payload } = message.contractCall;

  // we get the tx receipt 
  await provider.ready;
  const receipt = await provider.getTransactionReceipt(hash);
  if (!receipt) {
    throw new Error('receipt is not available yet');
  }

  const transactionMetadata = {
    ethAddress: receipt.from,
    payload
  };

  console.log(transactionMetadata);

  // ic

  // fromAddress is hex string prefixed with 0x
  const fromAddresPid = Principal.fromHex(transactionMetadata.ethAddress.slice(2));
  const amountAsNat = BigInt(transactionMetadata.payload.amount);
  console.log('fromAddress', fromAddresPid);
  console.log("amountNat", amountAsNat);


  // send message to the proxy
  await eth_proxy.removeClaimable(fromAddresPid, amountAsNat);
};

export const main = sqsHandler<BlockNativePayload>(handleWithdraw, QUEUE_URL, undefined, 1);

