import { middyfy } from '../../libs/lambda';
import { EthProxies, queryClaimableMessages } from './claimableMessages';
import schema from './schema';
import { ValidatedEventQuery } from '../../libs/apiGateway';

export const ethProxyClaimableMessages: ValidatedEventQuery<typeof schema> = async (event): Promise<any> => {
  const response = await queryClaimableMessages(event, EthProxies.ETH_PROXY);
  return response;
};

export const main = middyfy(ethProxyClaimableMessages);
