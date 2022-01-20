const { expect } = require("chai");
const { ethers } = require("hardhat");

describe("Message hash", function () {
    it("Should return correct deposit message hash", async function () {
        // principal id hex form
        const canisterId = "0x00000000003000f10101";
        const principalId =
            "0xced2c72d7506fa87cd9c9d5e7e08e3614221272516ba4c152047ead802";
        const ethProxyAddr = "0x1b864e1CA9189CFbD8A14a53A02E26B00AB5e91a";

        // reconstruct the withdraw message hash
        const depositPayload = [principalId, 69000000];

        // 0xdc64a140aa3e981100a9beca4e685f962f0cf6c9000000000000000000000000
        const depositMessageHash = ethers.utils.solidityKeccak256(
            ["uint256", "uint256", "uint256", "uint256", "uint256"],
            [ethProxyAddr, canisterId, depositPayload.length, ...depositPayload]
        );

        expect(depositMessageHash).equals(
            "0xbc979e70fa8f9743ae0515d2bc10fed93108a80a1c84450c4e79a3e83825fc45"
        );
    });

    it("Should return correct withdrawal message hash", async function () {
        // principal id hex form
        const canisterId = "0x00000000003000f10101";
        const ethProxyAddr = "0x60DC1a1FD50F1cdA1D44dFf69Cec3E5C065417e8";
        const receiverAddr = '0xfd82d7abAbC1461798deB5a5d9812603fdd650cc';

        // reconstruct the withdraw message hash
        // 0.001 eth 
        const withdrawPayload = [receiverAddr, 1000000];

        // 0xdc64a140aa3e981100a9beca4e685f962f0cf6c9000000000000000000000000
        const withdrawMessageHash = ethers.utils.solidityKeccak256(
            ["uint256", "uint256", "uint256", "uint256", "uint256"],
            [canisterId, ethProxyAddr, withdrawPayload.length, ...withdrawPayload]
        );

        expect(withdrawMessageHash).equals(
            "0x3c478b7a95e4b23fc1af0c5367296ec78f4d4b47382e7e3e2a37b46ad73fbaee"
        );
    });
});
