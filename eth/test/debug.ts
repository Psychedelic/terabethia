import { ethers } from "hardhat";

describe("DebugContract", function () {
  it("Should debug", async function () {
    // Starknet Messaging Protocol
    const dbg = await ethers.getContractFactory("DebugContract");

    const contract = await dbg.deploy();
    await contract.deployed();

    await contract.split(
      "0xd0379be15bb6f33737b756e512dad1e71226b31fa648da57811f930badf6c163"
    );
  });
});

// "payload": [
//   "2726517970423655752410510161822592796305026733148466366058932535696430620542",
//   "2"
// ]
