import "source-map-support/register";

import Web3 from "web3";
import { ethers } from "ethers";
import middy from "@middy/core";
import { Tera } from "@libs/dfinity";
import { config } from "@libs/config";
import type { SQSEvent } from "aws-lambda";
import { Principal } from "@dfinity/principal";
import { formatJSONResponse } from "@libs/apiGateway";
import { BlockNativePayload } from "@libs/blocknative";
import sqsBatch from "@middy/sqs-partial-batch-failure";
import sqsJsonBodyParser from "@middy/sqs-json-body-parser";

const web3 = new Web3();
const { INFURA_KEY, ALCHEMY_KEY } = config;

const typesArray = [
  { type: "uint256", name: "wow" },
  { type: "uint256", name: "meow" },
  { type: "uint256", name: "principal" },
  { type: "uint256", name: "amount" },
];

const providers = {
  Mainnet: [
    `https://mainnet.infura.io/v3/${INFURA_KEY}`,
    `https://eth-mainnet.alchemyapi.io/v2/${ALCHEMY_KEY}`,
  ],
  Goerli: ["https://goerli.infura.io/v3/8328044ef20647ca8cf95216e364e9cb"],
};

const getProvider = (url: string) =>
  new ethers.providers.StaticJsonRpcProvider(url);

const receiveMessageFromL1 = async (event: SQSEvent) => {
  const promises = event.Records.map(async ({ body }) => {
    const data = body as unknown as BlockNativePayload;

    let provider;

    try {
      provider = await Promise.any(providers["Goerli"].map(getProvider));
    } catch (error) {
      throw new Error(error);
    }

    const eventRecipt = await provider.getTransactionReceipt(data.hash);
    const { to: from, logs } = eventRecipt;
    const eventProps = web3.eth.abi.decodeParameters(
      typesArray,
      logs[0]?.data as string
    );

    try {
      // Move pid to config/env/constants
      // bridge canister id
      const to = Principal.fromText("tcy4r-qaaaa-aaaab-qadyq-cai");
      const response = await Tera.storeMessage(from, to, [
        // pid
        BigInt(eventProps.principal),
        // amount
        BigInt(eventProps.amount),
        // ethAddr
        BigInt(from),
      ]);

      return Promise.resolve(
        formatJSONResponse({
          statusCode: 200,
          body: { message: "success", response },
        })
      );
    } catch (error) {
      console.error(`Error SendMessageTera: ${(error as Error).message}`);
      return Promise.reject(error);
    }
  });

  return Promise.allSettled(promises);
};

export const main = middy(receiveMessageFromL1)
  .use(sqsJsonBodyParser())
  .use(sqsBatch());
