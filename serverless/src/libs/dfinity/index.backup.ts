import fetch from 'cross-fetch';
import {
  Actor,
  ActorSubclass,
  HttpAgent,
  HttpAgentOptions,
} from '@dfinity/agent';
import { Principal } from '@dfinity/principal';
import { Ed25519KeyIdentity } from '@dfinity/identity';

import TERA_FACTORY from './idls/tera/tera.did';
import TerabethiaService, {
  OutgoingMessage,
  Result,
  Result1,
} from './idls/tera/tera.d';

export interface ActorParams {
  host: string;
  canisterId: string;
}

const createActor = ({
  host,
  canisterId,
}: ActorParams, privateKeyJson?: string): ActorSubclass<TerabethiaService> => {
  let identity: Ed25519KeyIdentity;

  if (privateKeyJson) {
    identity = Ed25519KeyIdentity.fromJSON(privateKeyJson);
  } else {
    identity = Ed25519KeyIdentity.generate();
  }

  const agent = new HttpAgent({
    host,
    fetch,
    identity,
  } as unknown as HttpAgentOptions);

  if (process.env.NODE_ENV !== 'production') {
    try {
      agent.fetchRootKey();
    } catch (err) {
      console.warn(
        'Oops! Unable to fetch root key, is the local replica running?',
      );
      console.error(err);
    }
  }

  return Actor.createActor<TerabethiaService>(TERA_FACTORY, {
    agent,
    canisterId,
  });
};
export class Terabethia {
  private actor: ActorSubclass<TerabethiaService>;

  constructor(canisterId: string, privateKeyJson?: string, host = 'https://ic0.app') {
    this.actor = createActor({
      host,
      canisterId,
    }, privateKeyJson);
  }

  storeMessage(
    from: Principal,
    to: Principal,
    payload: bigint[],
  ): Promise<Result1> {
    return this.actor.store_message(from, to, payload);
  }

  getMessages(): Promise<OutgoingMessage[]> {
    return this.actor.get_messages();
  }

  removeMessages(messages: Array<bigint>): Promise<Result> {
    return this.actor.remove_messages(messages);
  }
}
