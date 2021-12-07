import fetch from "cross-fetch";
import {
  Actor,
  ActorSubclass,
  HttpAgent,
  HttpAgentOptions,
} from "@dfinity/agent";
import { Principal } from "@dfinity/principal";
import { Ed25519KeyIdentity } from "@dfinity/identity";

import TERA_FACTORY from "./idls/tera/tera.did";
import _TERA_SERVICE, {
  OutgoingMessage,
  Result,
  Result_1,
} from "./idls/tera/tera";
import { config } from "@libs/config";

export interface ActorParams {
  host: string;
  canisterId: string;
  idlFactory: IdlFactory;
}

export const Hosts = {
  mainnet: "https://ic0.app",
  local: "http://localhost:8000",
};

type IdlFactory = ({ IDL }: { IDL: any }) => any;

const createActor = <T>({
  host,
  canisterId,
  idlFactory,
}: {
  host: string;
  canisterId: string;
  idlFactory: IdlFactory;
}): ActorSubclass<T> => {
  let identity = Ed25519KeyIdentity.generate();

  if (config.TERA_AGENT_KEY_PAIR) {
    identity = Ed25519KeyIdentity.fromJSON(config.TERA_AGENT_KEY_PAIR);
  }

  console.log('id pid', identity.getPrincipal().toText());

  const agent = new HttpAgent({
    host,
    fetch,
    identity,
  } as unknown as HttpAgentOptions);

  if (process.env.NODE_ENV !== "production") {
    try {
      agent.fetchRootKey();
    } catch (err) {
      console.warn(
        "Oops! Unable to fetch root key, is the local replica running?"
      );
      console.error(err);
    }
  }

  return Actor.createActor(idlFactory, {
    agent,
    canisterId,
  });
};

const teraCanister = createActor<_TERA_SERVICE>({
  host: Hosts.mainnet,
  canisterId: config.TERA_CANISTER_ID,
  idlFactory: TERA_FACTORY,
});

export const Tera = {
  storeMessage: async (
    from: Principal,
    to: Principal,
    payload: Array<bigint>
  ): Promise<Result_1> => {
    return await teraCanister.store_message(from, to, payload);
  },
  getMessages: async (): Promise<OutgoingMessage[]> => {
    return await teraCanister.get_messages();
  },
  removeMessages: async (messages: Array<bigint>): Promise<Result> => {
    return await teraCanister.remove_messages(messages);
  },
};
