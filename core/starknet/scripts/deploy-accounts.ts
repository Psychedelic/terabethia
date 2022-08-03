import hre, { starknet } from "hardhat";
import { OpenZeppelinAccount } from "@shardlabs/starknet-hardhat-plugin/dist/src/account";
import { handleAccountContractArtifacts } from "@shardlabs/starknet-hardhat-plugin/dist/src/account-utils";

(async () => {
  const contractPath = await handleAccountContractArtifacts(
    OpenZeppelinAccount.ACCOUNT_TYPE_NAME,
    OpenZeppelinAccount.ACCOUNT_ARTIFACTS_NAME,
    OpenZeppelinAccount.VERSION,
    hre
  );

  const accountContractFactory = await starknet.getContractFactory(
    contractPath
  );

  const adminContract = await accountContractFactory.deploy(
    {
      public_key: BigInt(
        "0xf4ce1607b79b6f0503656dcc911913afcab2ed1d9e1d3f0dab905907d1f7d0"
      ),
    },
    {}
  );

  console.log(adminContract.deployTxHash);
  console.log("deployed admin contract at: %s", adminContract.address);

  const operatorContract = await accountContractFactory.deploy(
    {
      public_key: BigInt(
        "0x29f36327de46bf61d2f6ea0e55f76031146227a5a9344fa27e79123ea91bee"
      ),
    },
    {}
  );

  console.log(operatorContract.deployTxHash);
  console.log("deployed operator contract at: %s", operatorContract.address);
})();
