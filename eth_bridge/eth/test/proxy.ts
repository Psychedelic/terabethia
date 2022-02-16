import { ethers } from "hardhat";

const ethValue1 = ethers.utils.parseEther("0.069");

const overrides = {
  // To convert Ether to Wei:
  value: ethValue1,
};

describe("Eth Proxy", function () {
  it("Should deposit some Ethereum", async function () {
    const EthProxy = await ethers.getContractFactory("EthProxy");

    // const ethProxy = await EthProxy.deploy(tera.address);

    // // principal id hex form
    // const principalId = '0xced2c72d7506fa87cd9c9d5e7e08e3614221272516ba4c152047ead802';

    // // deposit validation
    // const depositTx = await ethProxy.deposit(principalId, overrides);
    // await depositTx.wait();
    // const balance = await ethers.provider.getBalance(ethProxy.address);
    // expect(balance).equals(ethValue1);
  });
});
