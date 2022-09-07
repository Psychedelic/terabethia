import { formatJSONResponse, ValidatedEventQuery } from '@libs/apiGateway';
import StarknetDatabase from '@libs/dynamo/starknet';
import { ethAddressAsPrincipalBN, requireEnv } from '@libs/utils/utils';
import schema from './schema';

const envs = requireEnv([
  'STARKNET_TABLE_NAME',
  'AWS_STAGE',
]);

export enum EthProxies {
  ETH_PROXY = '0x2E130E57021Bb4dfb95Eb4Dd0dD8CFCeB936148a',
  ERC20_PROXY = '0x8CA1651eadeF97D3aC36c25DAE4A552c1368F27d',
}

export const queryClaimableMessages = async (event: ValidatedEventQuery<typeof schema>, ethContract: EthProxies) => {
  if (!event || !event.queryStringParameters || !event.queryStringParameters.ethAddress) {
    return formatJSONResponse({
      statusCode: 400,
      body: { message: 'missing ethAddress in query params' },
    });
  }

  const { ethAddress } = event.queryStringParameters;

  const receiverAddress = ethAddressAsPrincipalBN(ethAddress);
  const proxyAddress = ethAddressAsPrincipalBN(ethContract);

  const db = new StarknetDatabase(envs.STARKNET_TABLE_NAME);

  try {
    const messages = await db.getMessagesForEthAddress(receiverAddress, proxyAddress);

    return formatJSONResponse({
      statusCode: 200,
      body: { message: messages },
    });
  } catch (e) {
    console.log(e);

    return formatJSONResponse({
      statusCode: 500,
      body: { message: 'Could not get messages' },
    });
  }
};
