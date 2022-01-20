import { ScheduledHandler } from "aws-lambda";
import { Tera } from "@libs/dfinity";
import { createPayload, updateState } from "@libs/eth";
import { DynamoDb } from "@libs/dynamo";

const { OPERATOR_PRIVATE_KEY, CONTRACT_ADDRESS } = process.env;

if (!OPERATOR_PRIVATE_KEY) {
  throw new Error("OPERATOR_PRIVATE_KEY must be set");
}

if (!CONTRACT_ADDRESS) {
  throw new Error("CONTRACT_ADDRESS must be set");
}

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
    .map((m) => `0x${m.hash}`);


  
  // skip empty payload
  if (messagesToL1.length == 0) {
    console.log("no messages in payload");
    return;
  }

  // we no longer handle consumed L2 messages thanks to nonce implementation
  
  // @todo: get nonce from Cairo contract
  // @todo: invoke Cairo send_message with arguments like:
  // send_message(nonce + 1, messagesToL1.length, messagesToL1)
  // write down tx hash, monitor for acceptance on L1

  // write lock on each message id when tx is submitted
  if (tx.hash) {
    const ids = await Promise.all(
      messages.map(async (m) => {
        const hid = m.id.toString(16);
        await db.setProcessingMessage(hid);
        return hid;
      })
    );

    await db.storeEthTransaction(tx.hash, ids);
  }
  // publish event that'll monitor tx
  // once the tx succeeds we should remove messages from the canister
};
