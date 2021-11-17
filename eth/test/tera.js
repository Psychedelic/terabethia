const { expect } = require("chai");
const { ethers } = require("hardhat");
const { soliditySha3 } = require("web3-utils");

const BN = require('bn.js');

const ethValue1 = ethers.utils.parseEther("5");
const ethValue2 = ethers.utils.parseEther("0.1");
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
    const b2 = await ethers.provider.getBalance(ethProxy.address);
    expect(b2).equals(ethValue1);

    const sequenceNumber = await tera.stateSequenceNumber();
    expect(sequenceNumber.toString()).equals('0');

    // const stateRoot = await tera.stateRoot();
    // expect(stateRoot).equals('0x0000000000000000000000000000000000000000000000000000000000000000');

    const txOperator = await tera.registerOperator('0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266');
    await txOperator.wait();


    // reconstruct the withdraw message hash
    const withdrawPayload = [
      '0x0000000000000000000000000000000000000000000000000000000000000000',
      numStringToBytes32(Buffer.from('f39fd6e51aad88f6f4ce6ab8827279cfffb92266', 'hex')),
      numStringToBytes32(ethValue2.toString()), // should be 0x000000000000000000000000000000000000000000000000016345785d8a0000
    ];

    expect(withdrawPayload[1]).equal('0x000000000000000000000000f39fd6e51aad88f6f4ce6ab8827279cfffb92266')
    expect(withdrawPayload[2]).equal('0x000000000000000000000000000000000000000000000000016345785d8a0000')

    const withdrawMessageHash = soliditySha3(
      "0x6d6e6932637a71616161616161616471616c3671636169000000000000000000",
      ethProxy.address,
      withdrawPayload.length,
      { t: 'bytes32', v: withdrawPayload }
    );

    // 0xefb80e98c9f7ac2ad55b3e4f5bb2d3a15fe8c187925eba2ffc721f74d1982c52
    expect(withdrawMessageHash).equals('0xe758a290839d36f05551ff7857b8c00b8052dc66d2b9eaf810d1ab9b029872a4');

    const updateStateTx = await tera.updateState(1, [
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

    const withdrawTx2 = await ethProxy.withdraw(ethValue2);

    await withdrawTx2.wait();

    const b3 = await ethers.provider.getBalance(ethProxy.address);
    expect(b3).equals(ethValue3);


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
