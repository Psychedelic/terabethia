import { ScheduledHandler } from "aws-lambda";
import { Tera } from "@libs/dfinity";
import { splitUint256 } from "@libs/utils";
import { DynamoDb } from "@libs/dynamo";
import TerabethiaStarknet from "@libs/starknet";
import bluebird from "bluebird";

const { OPERATOR_PRIVATE_KEY, CONTRACT_ADDRESS } = process.env;

if (!OPERATOR_PRIVATE_KEY) {
  throw new Error("OPERATOR_PRIVATE_KEY must be set");
}

if (!CONTRACT_ADDRESS) {
  throw new Error("CONTRACT_ADDRESS must be set");
}

const terabethia = new TerabethiaStarknet(
  "0x011215026475fe87b55d6638ee97b0113427d667f4a1d8a6cc8d0b573ea702af"
);

export const main: ScheduledHandler = async () => {
  const db = new DynamoDb();

  const rawMessages = await Tera.getMessages();

  const messages = await Promise.all(
    rawMessages.map(async (m) => {
      const isProcessing = await db.isProcessingMessage(m.id.toString(16));
      console.log({ isProcessing, hid: m.id.toString(16) });
      return { ...m, isProcessing };
    })
  ).then((messages) => messages.filter((m) => !m.isProcessing));

  const messagesToL1 = messages
    .filter((a) => a.produced)
    .map((m) => ({
      ...m,
      payload: splitUint256(m.hash),
    }));

  // skip empty payload
  if (messagesToL1.length == 0) {
    console.log("no messages in payload");
    return;
  }

  let nonce = await terabethia.getNonce();

  for (let message of messagesToL1) {
    nonce += 1;

    const [a, b] = message.payload;
    const hid = message.id.toString(16);
    let tx;
  
    try {
      tx = await terabethia.sendMessage(nonce.toString(), a, b);
    } catch (e) {
      console.log('error during starknet call');
      console.log(e);
      console.log(JSON.stringify(e.response));
      continue;
    }

    if (tx.transaction_hash) {
      await db.setProcessingMessage(hid);
      await db.storeTransaction(tx.transaction_hash, [hid]);

      // @todo: do we need to check for acceptance here?
      await bluebird.delay(2000);
    }
  }
};
