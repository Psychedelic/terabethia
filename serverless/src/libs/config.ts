export const config = {
  SNS_URL: process.env.SNS_URL,
  AWS_STAGE: process.env.AWS_STAGE,
  IS_OFFLINE: process.env.IS_OFFLINE,
  AWS_ACCOUNT_ID_LOCAL: '123456789012',
  TERA_QUEUE_URL: process.env.QUEUE_URL,
  AWS_ACCOUNT_ID: process.env.AWS_ACCOUNT_ID,
  INFURA_KEY: '8328044ef20647ca8cf95216e364e9cb',
  ALCHEMY_KEY: '8uppuN2k88ZIrJleq7uVcQLqIuedvAO6',
  TERA_CANISTER_ID: 's5qpg-tyaaa-aaaab-qad4a-cai',
  DYNAMO_LOCAL_PORT: process.env.DYNAMO_LOCAL_PORT,
  AWS_REGION: process.env.AWS_REGION || 'us-west-2',
  ETH_PROXY_CANISTER_ID: 'tcy4r-qaaaa-aaaab-qadyq-cai',
  TERA_AGENT_KEY_PAIR: process.env.TERA_AGENT_KEY_PAIR,
  ETH_L1_MESSAGE_TOPIC_ARN: process.env.ETH_L1_MESSAGE_TOPIC_ARN,
  ETH_L1_MESSAGE_TOPIC_NAME: process.env.ETH_L1_MESSAGE_TOPIC_NAME,
  PROVIDERS: {
    Mainnet: [
      `https://mainnet.infura.io/v3/${INFURA_KEY}`,
      `https://eth-mainnet.alchemyapi.io/v2/${ALCHEMY_KEY}`,
    ],
    Goerli: ['https://goerli.infura.io/v3/8328044ef20647ca8cf95216e364e9cb'],
  },
};
