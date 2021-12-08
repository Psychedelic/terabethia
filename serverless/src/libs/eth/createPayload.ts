import * as ethers from "ethers";

/**
 * Combines messages to L1 with messages to L2
 * @param messagesToL1  MessageType[]
 * @param messagesToL2  MessageType[]
 * @returns MessageType[]
 */
export const createPayload = (
  messagesToL1: string[],
  messagesToL2: string[]
): string[] => {
  const p = [
    ethers.utils.hexZeroPad(ethers.utils.hexlify(messagesToL1.length), 32),
    ...messagesToL1,
    ethers.utils.hexZeroPad(ethers.utils.hexlify(messagesToL2.length), 32),
    ...messagesToL2,
  ];

  return p.map((d) => ethers.utils.defaultAbiCoder.encode(["bytes32"], [d]));
};
