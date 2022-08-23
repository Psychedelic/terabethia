import { expect } from "chai";
import { ethers, upgrades } from "hardhat";
import { PayableOverrides } from "ethers/lib/ethers";
import {
  EthProxy__factory as EthProxyfactory,
  EthProxy,
  Terabethia__factory as TerabethiaFactory,
  Terabethia,
} from "../typechain";

const STARKNET_CONTRACT = "0xde29d060D45901Fb19ED6C6e959EB22d8626708e";

const ethValue1 = ethers.utils.parseEther("0.069");

const overrides: PayableOverrides & { from?: string | Promise<string> } = {
  // To convert Ether to Wei:
  value: ethValue1,
};

describe("Eth Proxy", () => {
  describe("Deployment", () => {
    let ethProxy: EthProxy;

    beforeEach(async () => {
      // We get the contract to deploy
      const EthProxy = (await ethers.getContractFactory(
        "EthProxy"
      )) as EthProxyfactory;

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

      ethProxy = await EthProxy.deploy(terabethia.address);
    });

    it("Should have balance of 0 on init", async () => {
      expect(await ethers.provider.getBalance(ethProxy.address)).equals(
        ethers.utils.parseEther("0")
      );
    });
  });

  describe("Transfers", () => {
    let ethProxy: EthProxy;

    beforeEach(async () => {
      // We get the contract to deploy
      const EthProxy = (await ethers.getContractFactory(
        "EthProxy"
      )) as EthProxyfactory;

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

      ethProxy = await EthProxy.deploy(terabethia.address);
    });

    it("Should deposit correct amount", async () => {
      // principal id hex form
      const principalId =
        "0xced2c72d7506fa87cd9c9d5e7e08e3614221272516ba4c152047ead802";

      // deposit validation
      const depositTx = await ethProxy.deposit(principalId, overrides);
      await depositTx.wait();
      const balance = await ethers.provider.getBalance(ethProxy.address);
      console.log(balance);
      expect(balance).equals(ethValue1);
    });
  });
});
