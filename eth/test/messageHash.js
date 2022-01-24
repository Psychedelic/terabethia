const { expect } = require("chai");
const { ethers } = require("hardhat");

describe("Message hash", function () {
    it("Should return correct deposit message hash", async function () {
        const nonce = "0x04";
        // principal id hex form
        const canisterId = "0x00000000003000f10101";
        const principalId =
            "0xced2c72d7506fa87cd9c9d5e7e08e3614221272516ba4c152047ead802";
        const ethProxyAddr = "0x1b864e1CA9189CFbD8A14a53A02E26B00AB5e91a";

        // reconstruct the withdraw message hash
        const depositPayload = [principalId, 69000000];

        // 0xdc64a140aa3e981100a9beca4e685f962f0cf6c9000000000000000000000000
        const depositMessageHash = ethers.utils.solidityKeccak256(
            ["uint256", "uint256", "uint256", "uint256", "uint256", "uint256"],
            [ethProxyAddr, canisterId, nonce, depositPayload.length, ...depositPayload]
        );

        expect(depositMessageHash).equals(
            "0xc9e23418a985892acc0fa031331080bfce112bdf841a3ae04a5181c6da1610b1"
        );
    });

    it("Should return correct withdrawal message hash", async function () {
        // principal id hex form
        const canisterId = "0x00000000003000f10101";
        const ethProxyAddr = "0xFa7FC33D0D5984d33e33AF5d3f504E33a251d52a";
        const receiverAddr = '0xfd82d7abAbC1461798deB5a5d9812603fdd650cc';

        // reconstruct the withdraw message hash
        // 0.001 eth 
        const withdrawPayload = [receiverAddr, 1000000];

        // 0xdc64a140aa3e981100a9beca4e685f962f0cf6c9000000000000000000000000
        const withdrawMessageHash = ethers.utils.solidityKeccak256(
            ["uint256", "uint256", "uint256", "uint256", "uint256"],
            [canisterId, ethProxyAddr, withdrawPayload.length, ...withdrawPayload]
        );

        // this is the hash we should send from the IC -> L1
        expect(withdrawMessageHash).equals(
            "0xd0379be15bb6f33737b756e512dad1e71226b31fa648da57811f930badf6c163"
        );

        const cairoTerabethia = '0x07147c37056a60424a7edaff294847f706979c6d3a53eef2b4412d3dbf9eb77c';
        const terabethiaProxy = "0x60DC1a1FD50F1cdA1D44dFf69Cec3E5C065417e8";

        const starknetMessageHash = ethers.utils.solidityKeccak256(
            ["uint256", "uint256", "uint256", "uint256", "uint256"],
            [cairoTerabethia, terabethiaProxy, 2, "24127044263607486132772889641222586723", "276768161078691357748506014484008718823"]
        );

        // this is the final hash that appears on Starknet L1 Contract
        expect(starknetMessageHash).equals(
            "0xbebedf4dff2fec23e14c1f9d715bd8bae1b2ca404bd0507097c7bee45223e371"
        );

    });
});