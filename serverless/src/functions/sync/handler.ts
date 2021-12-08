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
      console.log({isProcessing, hid: m.id.toString(16) })
      return { ...m, isProcessing };
    })
  ).then((messages) => messages.filter((m) => !m.isProcessing));

  const messagesToL1 = messages.filter((a) => a.produced).map((m) => `0x${m.hash}`);
  const messagesToL2 = messages.filter((a) => !a.produced).map((m) => `0x${m.hash}`);

  const payload = createPayload(messagesToL1, messagesToL2);

  // skip empty payload
  if(payload.length == 2) {
    console.log('no messages in payload');
    return;
  } 

  const tx = await updateState(OPERATOR_PRIVATE_KEY, CONTRACT_ADDRESS, payload);

  // write lock on each message id when tx is submitted
  if(tx.hash) {
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
