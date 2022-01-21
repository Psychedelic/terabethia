/**
 * Splits Uint256 into two Uint128
 * 
 * @param hash 
 * @returns
 */
export const splitUint256 = (hexString: string): BigInt[]=> {
    const buff = Buffer.from(hexString, 'hex');

    if(buff.length !== 32) {
        throw Error('string is not uint256');
    }

    // uint256(uint128(msgInt >> 128))
    const a = BigInt(`0x${buff.slice(0, 16).toString('hex')}`);

    // uint256(uint128(msgInt))
    const b = BigInt(`0x${buff.slice(16, 32).toString('hex')}`);
    return [a, b];
}