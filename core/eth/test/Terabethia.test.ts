import { expect } from "chai";
import { ethers, upgrades } from "hardhat";
import { SignerWithAddress } from "@nomiclabs/hardhat-ethers/signers";
import { anyValue } from "@nomicfoundation/hardhat-chai-matchers/withArgs";
import { Terabethia__factory as TerabethiaFactory, Terabethia } from "../typechain";

const STARKNET_CONTRACT = "0xde29d060D45901Fb19ED6C6e959EB22d8626708e";

describe("Terabethia", () => {
  describe("Deployment", () => {
    let tera: Terabethia;

    beforeEach(async () => {
      // We get the contract to deploy
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

      tera = await terabethia.deployed();
    });

    it("Should have 0 messages", async () => {
      expect(await tera.messages.length).to.equal(0);
    });
  });

  describe("Sending Messages", () => {
    let tera: Terabethia;
    let owner: SignerWithAddress;
    let addr1: SignerWithAddress;

    beforeEach(async () => {
      [owner, addr1] = await ethers.getSigners();

      // We get the contract to deploy
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

      tera = await terabethia.deployed();
    });

    it("Should send message", async () => {
      const toAddress = BigInt(addr1.address);
      const payload = [BigInt(0), BigInt(1)];

      expect(await tera.sendMessage(toAddress, payload)).to.not.be.revertedWith(
        "Send Message Reverted"
      );
    });
  });

  describe("Events", () => {
    let tera: Terabethia;
    let addr1: SignerWithAddress;

    beforeEach(async () => {
      [addr1] = await ethers.getSigners();

      // We get the contract to deploy
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

      tera = await terabethia.deployed();
    });

    it("Should emit an event on sendMessage", async function () {
      const toAddress = BigInt(addr1.address);
      const payload = [BigInt(0), BigInt(1)];

      await expect(await tera.sendMessage(toAddress, payload))
        .to.emit(tera, "LogMessageToL2")
        .withArgs(anyValue, anyValue, anyValue, anyValue);
    });
  });
});
