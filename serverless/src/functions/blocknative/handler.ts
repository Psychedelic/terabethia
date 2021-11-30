import "source-map-support/register";

import Web3 from "web3";
import { ethers } from "ethers";
import { config } from "@libs/config";
import { middyfy } from "@libs/lambda";
import { Tera } from "@libs/dfinity";
import { APIGatewayProxyHandler } from "aws-lambda";
import { formatJSONResponse } from "@libs/apiGateway";
import { BlockNativePayload } from "@libs/blocknative";
import { SNSClient, PublishCommand } from "@aws-sdk/client-sns";
import { SQSClient, SendMessageCommand } from "@aws-sdk/client-sqs";
import { IDL } from "@dfinity/candid";

const web3 = new Web3();

const typesArray = [
  { type: "uint256", name: "wow" },
  { type: "uint256", name: "meow" },
  { type: "uint256", name: "principal" },
  { type: "uint256", name: "doge" },
];

const providers = [
  // Mainnet
  // `https://mainnet.infura.io/v3/${config.INFURA_KEY}`,
  // `https://eth-mainnet.alchemyapi.io/v2/${config.ALCHEMY_KEY}`,
  "https://goerli.infura.io/v3/8328044ef20647ca8cf95216e364e9cb",
];

const teraL1MockTxn: BlockNativePayload = {
  hash: "0xe83bbfbebfd35f5e44a246372605edcdff2d087e3c89007a86404c1403170f3c",
};

const QueueUrl = config.TERA_QUEUE_URL;
const snsClient = new SNSClient({ region: config.AWS_REGION });
const sqsClient = new SQSClient({ region: config.AWS_REGION });
const getProvider = (url: string) =>
  new ethers.providers.StaticJsonRpcProvider(url);

export const blockNativeEventHook: APIGatewayProxyHandler = async (
  event
): Promise<any> => {
  // if (!event.body) {
  //   return formatJSONResponse({
  //     statusCode: 500,
  //     body: `Error blocknative hook: no data recieved!`,
  //   });
  // }

  let provider;

  try {
    provider = await Promise.any(providers.map(getProvider));
  } catch (error) {
    throw new Error(error);
  }

  // const teraL1Txn = event.body as unknown as BlockNativePayload;
  const teraL1Txn = teraL1MockTxn;
  const eventRecipt = await provider.getTransactionReceipt(teraL1Txn.hash);
  const response = {
    statusCode: 200,
    body: "",
  };

  const { to, logs } = eventRecipt;
  const eventProps = web3.eth.abi.decodeParameters(
    typesArray,
    logs[0]?.data as string
  );

  // const snsTopicPayload = {
  //   TopicArn: "TOPIC_ARN",
  //   Message: JSON.stringify(eventLogs),
  // };

  try {
    const response = await Tera.triggerCall(
      to,
      eventProps.principal,
      IDL.encode([IDL.Vec(IDL.Nat8)], [logs[0]?.data])
    );
    return response;

    // const command = new PublishCommand(snsTopicPayload);
    // const response = await snsClient.send(command);

    // return response;
    // const command = new SendMessageCommand({
    //   QueueUrl,
    //   MessageBody: JSON.stringify(eventLogs),
    // });
    // await sqsClient.send(command);
  } catch (e) {
    console.error("Exception on queue", e);
    response.body = `Error on send queue: ${e}`;
    response.statusCode = 500;
  }

  return formatJSONResponse(response);
};

export const main = middyfy(blockNativeEventHook);
