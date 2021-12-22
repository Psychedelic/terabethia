![Frame 644 (1)](https://user-images.githubusercontent.com/73345016/144523142-e67d4d3e-ae29-4e52-921e-b74fb64a57bb.png)
# Terabethia - A Bridge Between Ethereum & the Internet Computer

Terabethia is a bridge between Ethereum & the Internet Computer  that contracts in both chains can use to message each other, and that Ethereum assets (ERC20, ERC721, & ERC1155) can use to be automatically mirrored & used on the IC through Terabethia's Magic Proxy & Token Factory, which will deploy wrapped versions of those assets programmatically with equivalent standards on the IC (DIP20, DIP721, DIP1155).

- [Website](https://terabethia.ooo/)
- [Twitter](https://twitter.com/terabethia_)
- [Discord](https://discord.gg/yVEcEzmrgm)
- [DIP20](https://github.com/Psychedelic/DIP20)
- [DIP721](https://github.com/Psychedelic/DIP721)

On this repository you can find early iterations of:

- **eth:** The Ethereum side of the Terabethia bridge and its contract.
- **ic:** The Internet Computer side of the Terabethia bridge and its contract.
- **serverless**: The base serverless AWS service utilized by the bridge.

----

## Terabethia's Testnet - Getting Started

**Terabethia's Testnet is live for any developer to test!** This testnet uses the Ethereum Goerli testnet network in tandem with the Internet Computer to host the contracts for Terabethia's communication protocol.

You can start building on top of this testnet, communicating contracts across both chains. We've created an example of mirroring Goerli Ethereum to Wrapped Ethereum on the Internet Computer as a sample integration and potential showcase of Terabethia as an asset bridge:

- [Visit our Documentation to get started](https://docs.terabethia.ooo/)
- [View our ETH/WETH Example](https://docs.terabethia.ooo/terabethia-testnet/eth-weth/)

## Terabethia's Bridge Protocol Architecture 
![Group 5972 (3)](https://user-images.githubusercontent.com/73345016/144625840-621cbbed-d723-4624-be89-5f8aa69ce1f0.png)

As seen in the graphic above, Terabethia's architecture is composed of the following pieces:

- Terabethia (Ethereum): One end of the bridge, the protocol's contract on Ethereum.
- Terabethia (IC): The other end of the bridge. The protocol's contract on the IC.
- State Sync Infrastructure: An AWS/lambda (for now) infra to send messages Ethereum<->IC.*
- The Magic Proxies: Contracts for the Ethereum asset mirroring protocol.
Token Factory: Service in charge of deploying mirrored token contracts on the IC.

> The State Sync Infrastructure will be removed from AWS once the native IC/Ethereum integration goes live (at which point this piece can be built in a canister on the IC). However starting with this piece being centralized is common practice for many bridges.

## Terabethia's Magic Proxy Flow 
![Group 5974 (1)](https://user-images.githubusercontent.com/73345016/144625999-3098050f-ea08-413d-9176-0b1fb116db60.png)

One of the magic pieces is that it includes a built-in wrapped token factory on the Internet Computer that's in charge of deploying the mirrored ERC20 contracts to the IC using equivalent standards we've developed: DIP20, DIP, 721, & DIP1155 (coming soon).

There are two magic proxy contracts, one on Ethereum and another on the Internet Computer that can communicate with each other through Terabethia. 

They will automatically allow anyone to send ERC20 tokens (and soon ERC721 & ERC1155) to the IC and benefit from its L2-like benefits (low fees, fast transactions, etc.) and other unique features (enhanced compute/storage capabilities, hosting/serving front ends directly from smart contracts, native IC/ETH integration, etc.) without needing to modify or rewrite any contracts or code.
