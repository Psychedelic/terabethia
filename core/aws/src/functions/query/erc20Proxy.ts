import { middyfy } from '@libs/lambda';
import { EthProxies, queryClaimableMessages } from './claimableMessages';
import schema from './schema';
import { ValidatedEventQuery } from '../../libs/apiGateway';

export const erc20ProxyClaimableMessages: ValidatedEventQuery<typeof schema> = async (event): Promise<any> => {
  const response = await queryClaimableMessages(event, EthProxies.ERC20_PROXY);
  return response;
};

export const main = middyfy(erc20ProxyClaimableMessages);
