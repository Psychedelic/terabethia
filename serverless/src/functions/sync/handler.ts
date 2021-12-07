import { middyfy } from "@libs/lambda";
import { ScheduledHandler } from "aws-lambda";
import { Tera } from "@libs/dfinity";
import { createPayload } from "@libs/eth";

const sync: ScheduledHandler = async () => {
  const messages = await Tera.getMessages();

  const messagesToL1 = messages.filter(a => a.produced).map(m => m.hash);
  const messagesToL2 = messages.filter(a => !a.produced).map(m => m.hash);

  const payload = createPayload(messagesToL1, messagesToL2);

  // @todo: send update state tx
};

export const main = middyfy(sync);
