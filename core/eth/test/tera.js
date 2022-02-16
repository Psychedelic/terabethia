const { expect } = require("chai");
const { ethers, upgrades } = require("hardhat");
const { soliditySha3 } = require("web3-utils");

const BN = require('bn.js');

const ethValue1 = ethers.utils.parseEther("0.069");
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
    // Starknet Messaging Protocol
    const Starknet = await ethers.getContractFactory("Terabethia");

    // Upgradable Proxy
    // const Proxy = await ethers.getContractFactory("Proxy");

    // Ethereum Proxy which is using Tera (Upgradable Starknet Messaging Proxy)
    const EthProxy = await ethers.getContractFactory("EthProxy");

    const impl = await Starknet.deploy();
    await impl.deployed();

    // we only support sequenceNumber=1 as state init
    const initialState = ethers.utils.defaultAbiCoder.encode(['uint256'], [1]);
    console.log({ initialState });

    // const tera = await Proxy.deploy(300);
    const tera = await upgrades.deployProxy(Starknet, [initialState]);

    await tera.deployed();

    // const addImplTx = await tera.addImplementation(impl.address, initialState, false);
    // await addImplTx.wait();


    // const upgradeToTx = await tera.upgradeTo(impl.address, initialState, false);
    // await upgradeToTx.wait();

    // const implAddr = await tera.getImplementation();
    // expect(implAddr).equal(impl.address);

    const id = await tera.identify();
    expect(id).equals('Terabethia_2021_1');

    const ethProxy = await EthProxy.deploy(tera.address);



    // 5oynr-yl472-mav57-c2oxo-g7woc-yytib-mp5bo-kzg3b-622pu-uatef-uqe

    // principal id hex form
    const canisterId = '0x00000000003000f10101';
    const principalId = '0xced2c72d7506fa87cd9c9d5e7e08e3614221272516ba4c152047ead802';

    // deposit validation
    const depositTx = await ethProxy.deposit(principalId, overrides);
    await depositTx.wait();
    const balance = await ethers.provider.getBalance(ethProxy.address);
    expect(balance).equals(ethValue1);


    // withdrawing without permission should revert
    const withdrawTx = ethProxy.withdraw(ethValue2);
    await expect(withdrawTx).to.be.reverted;
    const b2 = await ethers.provider.getBalance(ethProxy.address);
    expect(b2).equals(ethValue1);

    const sequenceNumber = await tera.stateSequenceNumber();
    expect(sequenceNumber.toString()).equals('1');

    // const stateRoot = await tera.stateRoot();
    // expect(stateRoot).equals('0x0000000000000000000000000000000000000000000000000000000000000000');

    const txOperator = await tera.registerOperator('0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266');
    await txOperator.wait();


    // reconstruct the withdraw message hash
    const withdrawPayload = [
      // '0x00',
      '0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266',
      ethValue2.toString(), // should be 0x000000000000000000000000000000000000000000000000016345785d8a0000
    ];

    // expect(withdrawPayload[1]).equal('0x000000000000000000000000f39fd6e51aad88f6f4ce6ab8827279cfffb92266')
    // expect(withdrawPayload[2]).equal('0x000000000000000000000000000000000000000000000000016345785d8a0000')


    // 0xdc64a140aa3e981100a9beca4e685f962f0cf6c9000000000000000000000000

    console.log('ETH ADDR BYTES32', ethProxy.address.substr(2).toLowerCase());

    const withdrawMessageHash = soliditySha3(
      { t: 'uint256', v: canisterId },
      {
        t: 'uint256', v: ethProxy.address
      },
      {
        t: 'uint256', v: withdrawPayload.length
      },
      { t: 'uint256', v: withdrawPayload }
    );

    expect(withdrawMessageHash).equals('0xceb0c9e3cd46a643b206ecb20bdf18f436166aff9823fedb8f8ee02cc5561fba');

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

    const withdrawTx2 = await ethProxy.withdraw(ethValue2);

    await withdrawTx2.wait();

    const withdrawTx3 = ethProxy.withdraw(ethValue2);
    await expect(withdrawTx3).to.be.reverted;

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