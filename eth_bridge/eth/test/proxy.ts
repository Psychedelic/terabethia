import { expect } from "chai";
import { ethers, upgrades } from "hardhat";
import { BigNumber, PayableOverrides } from "ethers/lib/ethers";
import {
  EthProxy__factory as EthProxyfactory,
  EthProxy,
  Terabethia__factory as TerabethiaFactory,
  Terabethia,
} from "../typechain";

const STARKNET_CONTRACT = "0xde29d060D45901Fb19ED6C6e959EB22d8626708e";

const ethValue1 = ethers.utils.parseEther("0.069").toBigInt();
const ethValue2 = ethers.utils.parseEther("0.009").toBigInt();

const overrides: PayableOverrides & { from?: string | Promise<string> } = {
  // To convert Ether to Wei:
  value: ethValue1,
};

const deploy = async () => {
  // We get the contract to deploy
  const EthProxy = (await ethers.getContractFactory(
    "EthProxy"
  )) as EthProxyfactory;

  const Terabethia = (await ethers.getContractFactory(
    "Terabethia"
  )) as TerabethiaFactory;

  const impl = await Terabethia.deploy();
  await impl.deployed();

  // eslint-disable-next-line no-unused-vars
  const initialState = ethers.utils.defaultAbiCoder.encode(["uint256"], [1]);

  const terabethia = (await upgrades.deployProxy(Terabethia, [
    STARKNET_CONTRACT,
  ])) as Terabethia;

  return EthProxy.deploy(terabethia.address);
};

describe("Eth Proxy", () => {
  describe("Deployment", () => {
    let ethProxy: EthProxy;

    beforeEach(async () => {
      ethProxy = await deploy();
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
      ethProxy = await deploy();
    });

    it("Should deposit correct amount", async () => {
      // principal id hex form
      const principalId =
        "0xced2c72d7506fa87cd9c9d5e7e08e3614221272516ba4c152047ead802";

      // deposit validation
      const depositTx = await ethProxy.deposit(principalId, overrides);
      await depositTx.wait();
      const balance = await ethers.provider.getBalance(ethProxy.address);
      expect(balance).equals(ethValue1);
    });
  });


  describe("Pausable", () => {
    let ethProxy: EthProxy;

    beforeEach(async () => {
      ethProxy = await deploy();
    });

    it("Should allow to pause only exectued by the owner", async () => {
      const principalId =
        "0xced2c72d7506fa87cd9c9d5e7e08e3614221272516ba4c152047ead802";

      const [owner, user] = await ethers.getSigners();

      // deposit validation

      await expect(ethProxy.connect(user).pause()).to.be.revertedWith(
        "Ownable: caller is not the owner"
      );

      // eslint-disable-next-line no-unused-vars
      const pause = await ethProxy.connect(owner).pause();
      const paused = await ethProxy.paused();
      expect(paused).to.eq(true);

      await expect(ethProxy.deposit(principalId, overrides)).to.be.revertedWith(
        "Pausable: paused"
      );

      await expect(ethProxy.connect(user).unpause()).to.be.revertedWith(
        "Ownable: caller is not the owner"
      );

      // eslint-disable-next-line no-unused-vars
      const unpause = await ethProxy.connect(owner).unpause();
      const paused1 = await ethProxy.paused();

      expect(paused1).equals(false);
    });
  });
});
