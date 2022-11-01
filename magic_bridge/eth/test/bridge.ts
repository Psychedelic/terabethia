import { expect } from "chai";
import { ethers, upgrades } from "hardhat";
import {
  ERC20Bridge__factory as ERC20BridgeFactory,
  ERC20Bridge,
  Terabethia__factory as TerabethiaFactory,
  Terabethia,
} from "../typechain";
import { TestToken } from "../typechain/TestToken";

const ethValue1 = ethers.utils.parseEther("0.069");
const STARKNET_CONTRACT = "0xde29d060D45901Fb19ED6C6e959EB22d8626708e";

const deploy = async () => {
  // We get the contract to deploy
  const ERC20Bridge = (await ethers.getContractFactory(
    "ERC20Bridge"
  )) as ERC20BridgeFactory;

  const Terabethia = (await ethers.getContractFactory(
    "Terabethia"
  )) as TerabethiaFactory;

  const EthProxy = (await ethers.getContractFactory(
    "EthProxy"
  )) as EthProxyfactory;

  const impl = await Terabethia.deploy();
  await impl.deployed();

  // eslint-disable-next-line no-unused-vars
  const initialState = ethers.utils.defaultAbiCoder.encode(["uint256"], [1]);
  // console.log({ initialState });

  const terabethia = (await upgrades.deployProxy(Terabethia, [
    STARKNET_CONTRACT,
  ])) as Terabethia;

  const TestToken = await ethers.getContractFactory("TestToken");
  const token = TestToken.deploy(
    "10000000000000000000000"
  ) as Promise<TestToken>;

  const implToken = await token;
  const testToken = await implToken.deployed();

  const ethProxyImpl = await EthProxy.deploy(terabethia.address);
  const ethProxy = await ethProxyImpl.deployed();

  const erc20 = ERC20Bridge.deploy(
    terabethia.address,
    ethProxy.address,
    "0x326C977E6efc84E512bB9C30f76E30c160eD06FB"
  ) as Promise<ERC20Bridge>;

  const implErc20 = await erc20;
  // eslint-disable-next-line no-unused-vars
  const erc20Bridge = await implErc20.deployed();

  return Promise.all([token, erc20]);
};

describe("Eth Proxy", () => {
  describe("Deployment", () => {
    let testToken: TestToken;
    let erc20Bridge: ERC20Bridge;

    beforeEach(async () => {
      [testToken, erc20Bridge] = await deploy();
    });

    it("Should have balance of 0 on init", async () => {
      expect(await ethers.provider.getBalance(erc20Bridge.address)).equals(
        ethers.utils.parseEther("0")
      );
    });
  });

  describe("Transfers", async () => {
    let testToken: TestToken;
    let erc20Bridge: ERC20Bridge;

    beforeEach(async () => {
      [testToken, erc20Bridge] = await deploy();
    });

    it("Should deposit correct amount", async () => {
      // principal id hex form
      const principalId =
        "0xced2c72d7506fa87cd9c9d5e7e08e3614221272516ba4c152047ead802";

      const receipt = await testToken.approve(erc20Bridge.address, ethValue1);
      await receipt.wait();

      // deposit validation
      const depositTx = await erc20Bridge.deposit(
        testToken.address,
        ethValue1,
        principalId
      );
      await depositTx.wait();
      const balance = await testToken.balanceOf(erc20Bridge.address);
      expect(balance).equals(ethValue1);
    });
  });
});
