import fetch from 'cross-fetch';
import {
  Actor,
  ActorSubclass,
  HttpAgent,
  HttpAgentOptions,
  SignIdentity,
} from '@dfinity/agent';
import { Principal } from '@dfinity/principal';
import DIP20_PROXY_FACTORY from './dip20_proxy/dip20_proxy.did'
import DIP20ProxyService, {
  RemoveClaimableResponse,
} from './dip20_proxy/dip20_proxy';

export interface ActorParams {
  host: string;
  canisterId: string;
}

const createActor = ({
  host,
  canisterId,
}: ActorParams, identity: SignIdentity): ActorSubclass<DIP20ProxyService> => {
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

  return Actor.createActor<DIP20ProxyService>(DIP20_PROXY_FACTORY, {
    agent,
    canisterId,
  });
};
export class DIP20Proxy {
  private actor: ActorSubclass<DIP20ProxyService>;

  constructor(canisterId: string, identity: SignIdentity, host = ' http://127.0.0.1:8000/') {
    this.actor = createActor({
      host,
      canisterId,
    }, identity);
  }

  removeClaimable(
    eth_address: Principal,
    token_id: Principal,
    amount: bigint,
  ): Promise<RemoveClaimableResponse> {
    return this.actor.remove_claimable(eth_address, token_id, amount);
  }
}
