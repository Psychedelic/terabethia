import fetch from 'cross-fetch';
import {
  Actor,
  ActorSubclass,
  HttpAgent,
  HttpAgentOptions,
  SignIdentity,
} from '@dfinity/agent';
import { Principal } from '@dfinity/principal';
import MAGIC_BRIDGE_FACTORY from './magic_bridge/magic_bridge.did'
import MagicBridgeService from './magic_bridge/magic_bridge';

export interface ActorParams {
  host: string;
  canisterId: string;
}

const createActor = ({
  host,
  canisterId,
}: ActorParams, identity: SignIdentity): ActorSubclass<MagicBridgeService> => {
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

  return Actor.createActor<MagicBridgeService>(MAGIC_BRIDGE_FACTORY, {
    agent,
    canisterId,
  });
};
export class MagicBridge {
  private actor: ActorSubclass<MagicBridgeService>;

  constructor(canisterId: string, identity: SignIdentity, host = 'https://ic0.app/') {
    this.actor = createActor({
      host,
      canisterId,
    }, identity);
  }

  getPrincipal(
    eth_address_pid: Principal,
  ): Promise<[] | [Principal]> {
    return this.actor.get_canister(eth_address_pid);
  }
}