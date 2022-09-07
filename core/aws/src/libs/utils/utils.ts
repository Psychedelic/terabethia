import { Principal } from '@dfinity/principal';
import { BigNumber } from 'ethers';
import _ from 'lodash';

type EnvWithKeys<T extends string> = Record<T, string>;

const sliceToBigInt = (buff: Buffer, ...args: number[]): BigInt => BigInt(`0x${buff.slice(...args).toString('hex')}`);

/**
 * Splits Uint256 hex string into two Uint128 BigInts
 *
 * @param hash string
 * @returns BigInt[]
 */
export const splitUint256 = (hexString: string): [BigInt, BigInt] => {
  const buff = Buffer.from(hexString, 'hex');

  if (buff.length !== 32) {
    throw Error('string is not uint256');
  }

  // uint256(uint128(msgInt >> 128))
  const a = sliceToBigInt(buff, 0, 16);

  // uint256(uint128(msgInt))
  const b = sliceToBigInt(buff, -16);
  return [a, b];
};

export const ethAddressAsPrincipalBN = (ethAddress: string) => {
  const ethAddressAsArray = Principal.fromHex(ethAddress.slice(2)).toUint8Array();
  return BigNumber.from(ethAddressAsArray).toBigInt();
};

export const requireEnv = <T extends string>(names: T[]): EnvWithKeys<T> => {
  const envs = names.reduce((buff, name) => {
    if (!process.env[name]) {
      throw new Error(`${name} must be set`);
    }

    if (name === undefined) {
      return buff;
    }

    return _.set(buff, name, process.env[name]);
  }, {} as EnvWithKeys<T>);

  return envs;
};
