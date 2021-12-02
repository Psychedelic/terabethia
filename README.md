![og-tera-image](https://user-images.githubusercontent.com/73345016/144492052-88f41a6c-9578-4d02-861c-f9e655467330.png)
# Terabethia - A Bridge Between Ethereum & the Internet Computer

Terabethia is a bridge between Ethereum & the Internet Computer  that contracts in both chains can use to message each other, and that Ethereum assets (ERC20, ERC721, & ERC1155) can use to be automatically mirrored & used on the IC through Terabethia's Magic Proxy & Token Factory, which will deploy wrapped versions of those assets programmatically with equivalent standards on the IC (DIP20, DIP721, DIP1155).

- [Website](https://terabethia.ooo/)
- [Twitter](https://twitter.com/terabethia_)
- [Discord](https://discord.gg/yVEcEzmrgm)
- [DIP20](https://github.com/Psychedelic/DIP20)
- [DIP721](https://github.com/Psychedelic/DIP721)

> Important: This is an early look at Terabethia's repository, but it does not reference the final or upcoming test-net implementation. Don't consider the repository as a stable or complete release yet. We will further document, polish, and add-in key pieces to the Magic Proxy, the bridge, and more soon.

On this repository you can find early iterations of:

- **eth:** The Ethereum side of the Terabethia bridge and its contract.
- **ic:** The Internet Computer side of the Terabethia bridge and its contract.
- **serverless**: The base serverless AWS service utilized by the bridge.
