import { ethers, upgrades } from "hardhat";

const PROXY_ADDRESS = "0x60DC1a1FD50F1cdA1D44dFf69Cec3E5C065417e8";

async function main() {
  const [deployer] = await ethers.getSigners();

  console.log("using deployer", deployer);

  // We get the contract to deploy
  const Terabethia = await ethers.getContractFactory("Terabethia");

  const impl = await Terabethia.deploy();
  await impl.deployed();

  // we only support sequenceNumber=1 as state init
  const initialState = ethers.utils.defaultAbiCoder.encode(["uint256"], [1]);
  console.log({ initialState });

  const tera = await upgrades.upgradeProxy(PROXY_ADDRESS, Terabethia);
  await tera.deployed();

  console.log("Terabethia deployed to:", impl.address);
  console.log("Terabethia proxy deployed to:", tera.address);

  console.log("Execute these commands to verify contracts on Etherscan:");
  console.log(`npx hardhat verify --network goerli ${impl.address}`);
}

main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
