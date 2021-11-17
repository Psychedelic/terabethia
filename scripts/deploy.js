async function main() {
    // We get the contract to deploy
    const Starknet = await ethers.getContractFactory("Starknet");
    const contract = await Starknet.deploy();

    console.log("Starknet deployed to:", contract.address);
}

main()
    .then(() => process.exit(0))
    .catch((error) => {
        console.error(error);
        process.exit(1);
    });