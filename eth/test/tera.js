const { expect } = require("chai");
const { ethers } = require("hardhat");

const ethValue1 = ethers.utils.parseEther("5");
const ethValue2 = ethers.utils.parseEther("0.1");

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

    // 5oynryl472mav57c2oxog7wocyytibmp5bokzg3b622puuatefuqe

    // deposit validation
    const depositTx = await ethProxy.deposit(ethers.utils.formatBytes32String('mni2czqaaaaaaadqal6qcai'), overrides);
    await depositTx.wait();
    const balance = await ethers.provider.getBalance(ethProxy.address);
    expect(balance).equals(ethValue1)


    // withdrawing without permission should revert
    const withdrawTx = ethProxy.withdraw(ethValue2);
    await expect(withdrawTx).to.be.reverted;
    const balance = await ethers.provider.getBalance(ethProxy.address);
    expect(balance).equals(ethValue1);

    // const wr = await withdrawTx.wait();

    // console.log(wr);
    // console.log(JSON.stringify(wr));

    // const tx = await tera.sendMessageToL2(ethers.utils.formatBytes32String('mni2czqaaaaaaadqal6qcai'), 2, [0, 1]);
    // const r = await tx.wait();
    // console.log('tx mined', r);

    // const stateRoot = await tera.stateRoot();

    // console.log({ stateRoot });

    // const messages = await tera.l1ToL2Messages();

    // console.log('msgHash', { messages });
  });
});
