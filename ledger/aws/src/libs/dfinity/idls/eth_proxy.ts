import fetch from 'cross-fetch';
import {
  Actor,
  ActorSubclass,
  HttpAgent,
  HttpAgentOptions,
  SignIdentity,
} from '@dfinity/agent';
import { Principal } from '@dfinity/principal';
import ETH_PROXY_FACTORY from './eth_proxy/eth.did'
import EthProxyService, {
  RemoveClaimableResponse,
} from './eth_proxy/eth';

export interface ActorParams {
  host: string;
  canisterId: string;
}

const createActor = ({
  host,
  canisterId,
}: ActorParams, identity: SignIdentity): ActorSubclass<EthProxyService> => {
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

  return Actor.createActor<EthProxyService>(ETH_PROXY_FACTORY, {
    agent,
    canisterId,
  });
};
export class EthProxy {
  private actor: ActorSubclass<EthProxyService>;

  constructor(canisterId: string, identity: SignIdentity, host = 'https://ic0.app') {
    this.actor = createActor({
      host,
      canisterId,
    }, identity);
  }

  removeClaimable(
    eth_address: Principal,
    amount: bigint,
  ): Promise<RemoveClaimableResponse> {
    return this.actor.remove_claimable(eth_address, amount);
  }
}
