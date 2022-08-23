import 'source-map-support/register';

import { ethers } from 'ethers';
import { DIP20Proxy } from '@/libs/dfinity';
import { Principal } from '@dfinity/principal';
import { BlockNativePayload } from '@/libs/blocknative';
import { requireEnv, sqsHandler } from '@/libs/utils';

import { Secp256k1KeyIdentity } from '@dfinity/identity';
import { MagicBridge } from '@/libs/dfinity/idls/magic_bridge';

const envs = requireEnv([
  'ETHEREUM_PROVIDER_URL',
  'DIP20_PROXY_CANISTER_ID',
  'QUEUE_URL',
  'IC_IDENTITY',
  'MAGIC_BRIDGE_CANISTER_ID'
]);

// EthProxy ETH
const provider = new ethers.providers.StaticJsonRpcProvider(envs.ETHEREUM_PROVIDER_URL);

const principalArrayFromEthAddress = (ethAddress: string) => {
  const arr = ethers.utils.arrayify(ethAddress);
  const paddedArr = Array(29 - arr.length).fill(0);
  const concat = new Uint8Array([...paddedArr, ...arr]);
  return Principal.fromUint8Array(concat);
}


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
  const dip20_proxy = new DIP20Proxy(envs.DIP20_PROXY_CANISTER_ID, identity);
  const magicBridge = new MagicBridge(envs.MAGIC_BRIDGE_CANISTER_ID, identity);

  // fromAddress is hex string prefixed with 0x
  console.log('ethAddress:', transactionMetadata.ethAddress)
  const fromAddresPid = Principal.fromHex(transactionMetadata.ethAddress.slice(2));
  console.log('fromAddressPid:', fromAddresPid.toString());

  const amountAsNat = BigInt(transactionMetadata.payload.amount);

  const tokenAddressPid = principalArrayFromEthAddress(transactionMetadata.payload.token);
  const dip20CanisterPid = await magicBridge.getPrincipal(tokenAddressPid);
  const dip20Principal = Principal.fromText(dip20CanisterPid.toString());

  console.log('tokenAddressPid:', tokenAddressPid.toString());
  console.log('dip20CanisterId', dip20CanisterPid.toString());
  console.log("amountNat:", amountAsNat);

  // send message to the proxy
  await dip20_proxy.removeClaimable(fromAddresPid, dip20Principal, amountAsNat);
};

export const main = sqsHandler<BlockNativePayload>(handleWithdraw, envs.QUEUE_URL, undefined, 1);


