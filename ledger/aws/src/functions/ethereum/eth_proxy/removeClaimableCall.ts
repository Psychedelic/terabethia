import 'source-map-support/register';

import { ethers } from 'ethers';
import { EthProxy } from '@/libs/dfinity';
import { Principal } from '@dfinity/principal';
import { BlockNativePayload } from '@/libs/blocknative';
import { requireEnv, sqsHandler } from '@/libs/utils';

import { Secp256k1KeyIdentity } from '@dfinity/identity';

const envs = requireEnv([
  'ETHEREUM_PROVIDER_URL',
  'ETH_PROXY_CANISTER_ID',
  'QUEUE_URL',
  'ETHEREUM_CONTRACT',
  'IC_IDENTITY'
]);

// EthProxy ETH
const provider = new ethers.providers.StaticJsonRpcProvider(envs.ETHEREUM_PROVIDER_URL);


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

  // ic call
  const identity = Secp256k1KeyIdentity.fromJSON(envs.IC_IDENTITY)
  const eth_proxy = new EthProxy(envs.ETH_PROXY_CANISTER_ID, identity);

  // fromAddress is hex string prefixed with 0x
  const fromAddresPid = Principal.fromHex(transactionMetadata.ethAddress.slice(2));
  const amountAsNat = BigInt(transactionMetadata.payload.amount);
  console.log('fromAddress', fromAddresPid);
  console.log("amountNat", amountAsNat);


  // send message to the proxy
  await eth_proxy.removeClaimable(fromAddresPid, amountAsNat);
};

export const main = sqsHandler<BlockNativePayload>(handleWithdraw, envs.QUEUE_URL, undefined, 1);

