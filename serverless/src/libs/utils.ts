const sliceToBigInt = (buff: Buffer, ...args: number[]): BigInt => {
    return BigInt(`0x${buff.slice(...args).toString('hex')}`);
}

/**
 * Splits Uint256 hex string into two Uint128 BigInts
 * 
 * @param hash string
 * @returns BigInt[]
 */
export const splitUint256 = (hexString: string): [BigInt, BigInt]=> {
    const buff = Buffer.from(hexString, 'hex');

    if(buff.length !== 32) {
        throw Error('string is not uint256');
    }

    // uint256(uint128(msgInt >> 128))
    const a = sliceToBigInt(buff, 0, 16);

    // uint256(uint128(msgInt))
    const b = sliceToBigInt(buff, -16);
    return [a, b];
}