

const { expect } = require("chai");
const { ethers, upgrades } = require("hardhat");
const { soliditySha3 } = require("web3-utils");

const BN = require('bn.js');

const ethValue1 = ethers.utils.parseEther("5");
const ethValue2 = ethers.utils.parseEther("0.01");
const ethValue3 = ethers.utils.parseEther("4.9");

let overrides = {
    // To convert Ether to Wei:
    value: ethValue1

    // Or you can use Wei directly if you have that:
    // value: someBigNumber
    // value: 1234   // Note that using JavaScript numbers requires they are less than Number.MAX_SAFE_INTEGER
    // value: "1234567890"
    // value: "0x1234"

    // Or, promises are also supported:
    // value: provider.getBalance(addr)
};

function numStringToBytes32(num) {
    var bn = new BN(num).toTwos(256);
    return padToBytes32(bn.toString(16));
}

function bytes32ToNumString(bytes32str) {
    bytes32str = bytes32str.replace(/^0x/, '');
    var bn = new BN(bytes32str, 16).fromTwos(256);
    return bn.toString();
}

function padToBytes32(n) {
    while (n.length < 64) {
        n = "0" + n;
    }
    return "0x" + n;
}


describe("Terabethia", function () {
    it("Should return the new greeting once it's changed", async function () {
        // Starknet Messaging Protocol
        const tera = await ethers.getContractAt("Terabethia", '0x64437B21cE909058b5201B9b1c02E5603dB197E7');

        // Upgradable Proxy
        // const Proxy = await ethers.getContractFactory("Proxy");

        // Ethereum Proxy which is using Tera (Upgradable Starknet Messaging Proxy)
        const ethProxy = await ethers.getContractAt("EthProxy", '0xd2f69519458c157a14C5CAf4ed991904870aF834');

        const sequenceNumber = await tera.stateSequenceNumber();
        expect(sequenceNumber.toString()).equals('1');

        console.log('sequenceStateNumber', sequenceNumber);


        // reconstruct the withdraw message hash
        const withdrawPayload = [
            // '0x0000000000000000000000000000000000000000000000000000000000000000',
            numStringToBytes32(Buffer.from('fd82d7ababc1461798deb5a5d9812603fdd650cc', 'hex')),
            numStringToBytes32(ethValue2.toString()), // should be 0x000000000000000000000000000000000000000000000000016345785d8a0000
        ];

        // expect(withdrawPayload[1]).equal('0x000000000000000000000000f39fd6e51aad88f6f4ce6ab8827279cfffb92266')
        // expect(withdrawPayload[2]).equal('0x000000000000000000000000000000000000000000000000016345785d8a0000')

        const withdrawMessageHash = soliditySha3(
            "0x6d6e6932637a71616161616161616471616c3671636169000000000000000000",
            ethProxy.address,
            withdrawPayload.length,
            { t: 'bytes32', v: withdrawPayload }
        );

        // 0xefb80e98c9f7ac2ad55b3e4f5bb2d3a15fe8c187925eba2ffc721f74d1982c52
        // expect(withdrawMessageHash).equals('0xff76cffb15cc5fbb35ba768c1aa7a821ccd5e4901c4ff733ea941747a2a52413');

        console.log('updating state');

        const updateStateTx = await tera.updateState(2, [
            // @todo do we need merkle states at all?
            // merkle state update from
            // '0x0000000000000000000000000000000000000000000000000000000000000000',
            // // merkle state update to
            // '0x0000000000000000000000000000000000000000000000000000000000000001',

            // number of L2 -> L1 messages
            '0x0000000000000000000000000000000000000000000000000000000000000001', // 1 message
            withdrawMessageHash,

            // number of L1 -> L2 messages
            '0x0000000000000000000000000000000000000000000000000000000000000000', // no messages
        ]);

        await updateStateTx.wait();

        console.log('update state done');

        const withdrawTx2 = await ethProxy.withdraw(ethValue2);

        await withdrawTx2.wait();

        const withdrawTx3 = ethProxy.withdraw(ethValue2);
        await expect(withdrawTx3).to.be.reverted;
    });
});