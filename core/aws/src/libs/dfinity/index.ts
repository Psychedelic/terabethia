import fetch from 'cross-fetch';
import {
  Actor,
  ActorSubclass,
  HttpAgent,
  HttpAgentOptions,
  SignIdentity,
} from '@dfinity/agent';
import { Principal } from '@dfinity/principal';
import TERA_FACTORY from './idls/tera/tera.did';
import TerabethiaService, {
  ConsumeMessageResponse,
  OutgoingMessagePair,
  StoreMessageResponse,
} from './idls/tera/tera';

export { KMSIdentity } from './kms';

export interface ActorParams {
  host: string;
  canisterId: string;
}

const createActor = ({
  host,
  canisterId,
}: ActorParams, identity: SignIdentity): ActorSubclass<TerabethiaService> => {
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

  constructor(canisterId: string, identity: SignIdentity, host = 'https://ic0.app') {
    this.actor = createActor({
      host,
      canisterId,
    }, identity);
  }

  storeMessage(
    from: Principal,
    to: Principal,
    nonce: bigint,
    payload: bigint[],
  ): Promise<StoreMessageResponse> {
    return this.actor.store_message(from, to, nonce, payload);
  }

  getMessages(): Promise<OutgoingMessagePair[]> {
    return this.actor.get_messages();
  }

  removeMessages(messages: OutgoingMessagePair[]): Promise<ConsumeMessageResponse> {
    return this.actor.remove_messages(messages);
  }
}