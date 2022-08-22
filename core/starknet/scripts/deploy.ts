import { starknet } from "hardhat";

(async () => {
  const adminContractAddress =
    "0x02827aa4fb9f2501800d231b572b72cd2234baf6ec83d269756e8d08f0d21bdc";

  const operatorContractAddress =
    "0x00cf095ad5341fb85824221bc64419ab3453b7a3a9a0e5e176061a0a341a5af3";

  console.log("deployed operator contract at: %s", operatorContractAddress);

  const implFactory = await starknet.getContractFactory("terabethia");
  const implContract = await implFactory.deploy();

  console.log("deployed implementation contract at: %s", implContract.address);

  const proxyFactory = await starknet.getContractFactory("upgradable");

  const proxyContract = await proxyFactory.deploy({
    approved_admin: adminContractAddress,
    approved_operator: operatorContractAddress,
    implementation_addr: implContract.address,
    l1_contract_address: "0x60dc1a1fd50f1cda1d44dff69cec3e5c065417e8",
  });

  console.log("terabethia proxy deployed at: %s", proxyContract.address);
})();
