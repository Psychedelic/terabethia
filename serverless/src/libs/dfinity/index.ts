import fetch from "cross-fetch";
import {
  Actor,
  ActorSubclass,
  HttpAgent,
  HttpAgentOptions,
} from "@dfinity/agent";
import { Principal } from "@dfinity/principal";

import _TERA_SERVICE from "./idls/tera/tera";
import TERA_FACTORY from "./idls/tera/tera.did";

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
  const agent = new HttpAgent({
    host,
    fetch,
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

export const Tera = {
  triggerCall: async (
    from: string,
    to: Principal,
    payload: any
  ): Promise<undefined> => {
    const teraCanister = createActor<_TERA_SERVICE>({
      host: Hosts.mainnet,
      canisterId: "",
      idlFactory: TERA_FACTORY,
    });
    return await teraCanister.trigger_call(from, to, payload);
  },
};
