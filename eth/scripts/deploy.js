const { ethers, upgrades } = require("hardhat");

async function main() {
    const [deployer] = await ethers.getSigners();

    console.log('using deployer', deployer);

    // We get the contract to deploy
    const Starknet = await ethers.getContractFactory("Terabethia");

    const impl = await Starknet.deploy();
    await impl.deployed();

    // we only support sequenceNumber=1 as state init
    const initialState = ethers.utils.defaultAbiCoder.encode(['uint256'], [1]);
    console.log({ initialState });

    // const tera = await Proxy.deploy(300);
    const tera = await upgrades.deployProxy(Starknet, [initialState]);
    await tera.deployed();

    // set proxy
    const EthProxy = await ethers.getContractFactory("EthProxy");
    const ethProxy = await EthProxy.deploy(tera.address);

    console.log("Terabethia deployed to:", impl.address);
    console.log("Terabethia proxy deployed to:", tera.address);
    console.log("Eth Bridge deployed to:", ethProxy.address);

    // set operator (who can update tera state)
    const txOperator = await tera.registerOperator('0x5B21e6B8432432B4f4E2C86F87eb88c78986E882');
    await txOperator.wait();

    console.log('Execute these commands to verify contracts on Etherscan:');
    console.log(`npx hardhat verify --network goerli ${impl.address}`);
    console.log(`npx hardhat verify --network goerli ${ethProxy.address} ${tera.address}`);
}

main()
    .then(() => process.exit(0))
    .catch((error) => {
        console.error(error);
        process.exit(1);
    });