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

describe("Eth Proxy", () => {
  describe("Deployment", () => {
    let erc20Bridge: ERC20Bridge;

    beforeEach(async () => {
      // We get the contract to deploy
      const ERC20Bridge = (await ethers.getContractFactory(
        "ERC20Bridge"
      )) as ERC20BridgeFactory;

      const Terabethia = (await ethers.getContractFactory(
        "Terabethia"
      )) as TerabethiaFactory;

      const impl = await Terabethia.deploy();
      await impl.deployed();

      const initialState = ethers.utils.defaultAbiCoder.encode(
        ["uint256"],
        [1]
      );
      console.log({ initialState });

      const terabethia = (await upgrades.deployProxy(Terabethia, [
        STARKNET_CONTRACT,
      ])) as Terabethia;

      erc20Bridge = await ERC20Bridge.deploy(terabethia.address);
    });

    it("Should have balance of 0 on init", async () => {
      expect(await ethers.provider.getBalance(erc20Bridge.address)).equals(
        ethers.utils.parseEther("0")
      );
    });
  });

  describe("Transfers", () => {
    let testToken: TestToken;
    let erc20Bridge: ERC20Bridge;

    beforeEach(async () => {
      // We get the contract to deploy
      const ERC20Bridge = (await ethers.getContractFactory(
        "ERC20Bridge"
      )) as ERC20BridgeFactory;

      const Terabethia = (await ethers.getContractFactory(
        "Terabethia"
      )) as TerabethiaFactory;

      const impl = await Terabethia.deploy();
      await impl.deployed();

      const initialState = ethers.utils.defaultAbiCoder.encode(
        ["uint256"],
        [1]
      );
      console.log({ initialState });

      const terabethia = (await upgrades.deployProxy(Terabethia, [
        STARKNET_CONTRACT,
      ])) as Terabethia;

      erc20Bridge = await ERC20Bridge.deploy(terabethia.address);

      const TestToken = await ethers.getContractFactory("TestToken");
      const token = (await TestToken.deploy(
        "10000000000000000000000"
      )) as TestToken;

      testToken = await token.deployed();
    });

    it("Should deposit correct amount", async () => {
      // principal id hex form
      const principalId =
        "0xced2c72d7506fa87cd9c9d5e7e08e3614221272516ba4c152047ead802";

      await testToken.approve(erc20Bridge.address, ethValue1);

      // deposit validation
      const depositTx = await erc20Bridge.deposit(
        testToken.address,
        ethValue1,
        principalId
      );
      await depositTx.wait();
      const balance = await testToken.balanceOf(erc20Bridge.address);
      console.log(balance);
      expect(balance).equals(ethValue1);
    });
  });
});
