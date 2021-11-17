const { expect } = require("chai");
const { ethers } = require("hardhat");

describe("Terabethia", function () {
  it("Should return the new greeting once it's changed", async function () {
    const Starknet = await ethers.getContractFactory("Starknet");
    const EthProxy = await ethers.getContractFactory("EthProxy");
    const tera = await Starknet.deploy();
    await tera.deployed();

    const id = await tera.identify();

    expect(id).equals('InternetComputer_2021_1');

    const ethProxy = await EthProxy.deploy(tera.address);

    console.log('ethProxy deployed', ethProxy.address);

    const depositTx = await ethProxy.deposit(ethers.utils.formatBytes32String('mni2czqaaaaaaadqal6qcai'), 100);
    // .toString()

    const tx = await tera.sendMessageToL2(ethers.utils.formatBytes32String('mni2czqaaaaaaadqal6qcai'), 2, [0, 1]);
    const r = await tx.wait();
    console.log('tx mined', r);

    const stateRoot = await tera.stateRoot();

    console.log({ stateRoot });

    const messages = await tera.l1ToL2Messages();

    console.log('msgHash', { messages });
  });
});
