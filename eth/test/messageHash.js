const { expect } = require("chai");
const { ethers, upgrades } = require("hardhat");
const { soliditySha3 } = require("web3-utils");

const BN = require('bn.js');

const ethValue1 = ethers.utils.parseEther("5");
const ethValue2 = ethers.utils.parseEther("0.1");
const ethValue3 = ethers.utils.parseEther("4.9");

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
    it("Message hash flow", async function () {
        // reconstruct the withdraw message hash
        const withdrawPayload = [
            '0x0000000000000000000000000000000000000000000000000000000000000000',
            '0x000000000000000000000000f39fd6e51aad88f6f4ce6ab8827279cfffb92266',
            '0x000000000000000000000000000000000000000000000000016345785d8a0000',
        ];

        const payloadLenHex = numStringToBytes32(withdrawPayload.length);

        const withdrawMessageHash = soliditySha3(
            "0x6d6e6932637a71616161616161616471616c3671636169000000000000000000",
            "0x000000000000000000000000d2f69519458c157a14c5caf4ed991904870af834",
            payloadLenHex,
            { t: 'bytes32', v: withdrawPayload }
        );

        console.log(payloadLenHex);

        // 0xefb80e98c9f7ac2ad55b3e4f5bb2d3a15fe8c187925eba2ffc721f74d1982c52
        expect(withdrawMessageHash).equals('0xa0651ef3ef5db8ae814a37abf8e63cbe88d0194789edc362951825bd4b2c5c55');
    });

    it("Message hash flow (no payload)", async function () {
        // reconstruct the withdraw message hash
        const withdrawPayload = [
            '0x0000000000000000000000000000000000000000000000000000000000000000',
            '0x000000000000000000000000f39fd6e51aad88f6f4ce6ab8827279cfffb92266',
            '0x000000000000000000000000000000000000000000000000016345785d8a0000',
        ];

        const payloadLenHex = numStringToBytes32(withdrawPayload.length);

        const withdrawMessageHash = soliditySha3(
            "0x6d6e6932637a71616161616161616471616c3671636169000000000000000000",
            "0x000000000000000000000000d2f69519458c157a14c5caf4ed991904870af834",
            payloadLenHex,
            // ...withdrawPayload,
        );

        console.log(payloadLenHex);

        // 0xefb80e98c9f7ac2ad55b3e4f5bb2d3a15fe8c187925eba2ffc721f74d1982c52
        // expect(withdrawMessageHash).equals('0xa0651ef3ef5db8ae814a37abf8e63cbe88d0194789edc362951825bd4b2c5c55');
        expect(withdrawMessageHash).equals('0x633d9cdbe67a14e3a3f66e378e6a520e5e48e6ea5fa0008dd4396e57c6a557bb');
    });
});