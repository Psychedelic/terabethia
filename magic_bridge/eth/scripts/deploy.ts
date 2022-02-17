import { ethers } from "hardhat";

const TERABETHIA_CONTRACT = "0x60DC1a1FD50F1cdA1D44dFf69Cec3E5C065417e8";

async function main() {
  const [deployer] = await ethers.getSigners();

  console.log("using deployer", deployer);

  // set proxy
  const ERC20Bridge = await ethers.getContractFactory("ERC20Bridge");
  const bridge = await ERC20Bridge.deploy(TERABETHIA_CONTRACT);

  console.log("Eth Bridge deployed to:", bridge.address);

  console.log("Execute these commands to verify contracts on Etherscan:");
  console.log(
    `npx hardhat verify --network goerli ${bridge.address} ${TERABETHIA_CONTRACT}`
  );
}

main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
