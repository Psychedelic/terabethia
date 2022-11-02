pragma solidity ^0.8.0;

import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/token/ERC20/extensions/IERC20Metadata.sol";
import "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/security/Pausable.sol";
import "./ITerabethiaCore.sol";
import "./IWeth.sol";
import "./IEthProxy.sol";

contract ERC20Bridge is Ownable, Pausable {
    // Terabethia core contract.
    ITerabethiaCore terabethiaCore;

    // EthProxy contract.
    IEthProxy ethProxy;

    // Weth contract.
    IWeth weth;

    // L2 Canister address
    uint256 constant CANISTER_ADDRESS = 0x00000000003001540101;

    mapping(address => bool) tokenWhiteList;
    mapping(address => bool) tokenBlackList;
    bool allTokensAllowed;

    /**
      Initializes the contract state.
    */
    constructor(
        ITerabethiaCore terabethiaCore_,
        IEthProxy ethProxy_,
        IWeth weth_
    ) {
        terabethiaCore = terabethiaCore_;
        ethProxy = ethProxy_;
        weth = weth_;
        allTokensAllowed = false;
    }

    function withdraw(address token, uint256 amount) external whenNotPaused {
        // Construct the withdrawal message's payload.
        uint256[] memory payload = new uint256[](3);
        payload[0] = uint256(uint160(token));
        payload[1] = uint256(uint160(msg.sender));
        payload[2] = amount;

        // Consume the message from the IC
        // This will revert the (Ethereum) transaction if the message does not exist.
        terabethiaCore.consumeMessage(CANISTER_ADDRESS, payload);

        // withdraw erc20
        SafeERC20.safeTransfer(IERC20(token), msg.sender, amount);
    }

    function deposit(
        address token,
        uint256 amount,
        uint256 user
    ) external whenNotPaused {
        require(isTokenAllowed(token), "Token not allowed");
        require(!isBlackListed(token), "Token is BlackListed");

        // convert string -> bytes -> uint256
        uint256 tokenName = uint256(
            stringToBytes32(IERC20Metadata(token).name())
        );
        uint256 tokenSymbol = uint256(
            stringToBytes32(IERC20Metadata(token).symbol())
        );

        SafeERC20.safeTransferFrom(
            IERC20(token),
            msg.sender,
            address(this),
            amount
        );

        if (token == address(weth)) {
            // unwarp weth to eth
            weth.withdraw(amount);
            // ethProxy handles the payload and the sendMessage call.
            return ethProxy.deposit{value: amount}(user);
        }

        // Construct the deposit message's payload.
        uint256[] memory payload = new uint256[](6);
        payload[0] = uint256(uint160(token));
        payload[1] = user;
        payload[2] = amount;
        payload[3] = tokenName;
        payload[4] = tokenSymbol;
        payload[5] = uint256(IERC20Metadata(token).decimals());

        // Send the message to the IC
        terabethiaCore.sendMessage(CANISTER_ADDRESS, payload);
    }

    function send(
        address recipient,
        address token,
        uint256 amount
    ) external onlyOwner {
        require(recipient != address(0), "Cannot send to zero address");

        SafeERC20.safeTransfer(IERC20(token), recipient, amount);
    }

    function pause() public onlyOwner {
        _pause();
    }

    function unpause() public onlyOwner {
        _unpause();
    }

    receive() external payable {}

    fallback() external payable {}

    function stringToBytes32(string memory source)
        public
        pure
        returns (bytes32 result)
    {
        bytes memory tempEmptyStringTest = bytes(source);
        if (tempEmptyStringTest.length == 0) {
            return 0x0;
        }

        assembly {
            result := mload(add(source, 32))
        }
    }

    function allowAllTokens() external onlyOwner {
        allTokensAllowed = true;
    }

    function disallowAllTokens() external onlyOwner {
        allTokensAllowed = false;
    }

    function addTokenToWhiteList(address token) external onlyOwner {
        tokenWhiteList[token] = true;
    }

    function removeFromWhiteList(address token) external onlyOwner {
        require(isWhiteListed(token), "Token must be white listed");
        delete (tokenWhiteList[token]);
    }

    function addTokenToBlackList(address token) external onlyOwner {
        tokenBlackList[token] = true;
    }

    function removeFromBlackList(address token) external onlyOwner {
        require(isBlackListed(token), "Token must be black listed");
        delete (tokenBlackList[token]);
    }

    function isWhiteListed(address token) public view returns (bool) {
        bool tokenIsWhiteListed = tokenWhiteList[token];
        return tokenIsWhiteListed;
    }

    function isBlackListed(address token) public view returns (bool) {
        bool tokenIsBlackListed = tokenBlackList[token];
        return tokenIsBlackListed;
    }

    function isTokenAllowed(address token) public view returns (bool) {
        return isWhiteListed(token) || allTokensAllowed;
    }

    function areAllTokensAllowed() public view returns (bool) {
        return allTokensAllowed;
    }
}
