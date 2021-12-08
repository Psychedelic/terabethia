import * as dotenv from "dotenv";

import { HardhatUserConfig, task } from "hardhat/config";
import "@nomiclabs/hardhat-etherscan";
import "@openzeppelin/hardhat-upgrades";
import "@nomiclabs/hardhat-waffle";
import "@typechain/hardhat";
import "hardhat-gas-reporter";
import "solidity-coverage";

dotenv.config();

task("accounts", "Prints the list of accounts", async (taskArgs, hre) => {
  const accounts = await hre.ethers.getSigners();

  for (const account of accounts) {
    console.log(account.address);
  }
});

task("deploy", "Deploy the smart contracts", async (taskArgs, hre) => {
  const [deployer] = await hre.ethers.getSigners();

  console.log("using deployer", deployer);

  // We get the contract to deploy
  const Starknet = await hre.ethers.getContractFactory("Terabethia");

  const impl = await Starknet.deploy();
  await impl.deployed();

  // we only support sequenceNumber=1 as state init
  const initialState = hre.ethers.utils.defaultAbiCoder.encode(
    ["uint256"],
    [1]
  );
  console.log({ initialState });

  // const tera = await Proxy.deploy(300);
  const tera = await hre.upgrades.deployProxy(Starknet, [initialState]);
  await tera.deployed();

  // set proxy
  const EthProxy = await hre.ethers.getContractFactory("EthProxy");
  const ethProxy = await EthProxy.deploy(tera.address);

  console.log("Terabethia deployed to:", impl.address);
  console.log("Terabethia proxy deployed to:", tera.address);
  console.log("Eth Bridge deployed to:", ethProxy.address);

  // set operator (who can update tera state)
  const txOperator = await tera.registerOperator(
    "0x5B21e6B8432432B4f4E2C86F87eb88c78986E882"
  );
  await txOperator.wait();

  await hre.run("verify:verify", {
    address: impl.address,
  });

  await hre.run("verify:verify", {
    address: ethProxy.address,
  });

  await hre.run("verify:verify", {
    address: tera.address,
  });
});

const config: HardhatUserConfig = {
  solidity: "0.6.12",
  networks: {
    goerli: {
      url: process.env.ALCHEMY_ENDPOINT,
      accounts:
        process.env.PRIVATE_KEY !== undefined ? [process.env.PRIVATE_KEY] : [],
      gas: 2100000,
      gasPrice: 8000000000,
    },
  },
  gasReporter: {
    enabled: process.env.REPORT_GAS !== undefined,
    currency: "USD",
  },
  etherscan: {
    apiKey: process.env.ETHERSCAN_API_KEY,
  },
  // This is property is not a memebr of type {HardhatUserConfig}
  // abiExporter: {
  //   path: './data/abi',
  //   clear: true,
  //   flat: true,
  //   only: [],
  //   spacing: 2,
  //   pretty: true,
  // },
};

export default config;
