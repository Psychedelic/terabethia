import ethers from 'ethers';

export { createPayload } from './createPayload';

const abi = [
    "function stateSequenceNumber() view returns (int256)",
    "function updateState(int256 sequenceNumber, bytes32[] output)"
];

export const updateState = async (privateKey: string, contractAddress: string, payload: string[]) => {
    // Connect to the network
    const provider = ethers.getDefaultProvider('goerli');

    // The address from the above deployment example
    const wallet = new ethers.Wallet(privateKey, provider);

    // We connect to the Contract using a Provider, so we will only
    // have read-only access to the Contract
    const contract = new ethers.Contract(contractAddress, abi, provider);
    const contractWithSigner = contract.connect(wallet);

    const sequenceNumber = await contract.stateSequenceNumber();

    console.log('current sequence number', sequenceNumber);

    return contractWithSigner.updateState(sequenceNumber + 1, payload);
}