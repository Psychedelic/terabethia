pragma solidity ^0.8.0;

import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/token/ERC20/extensions/IERC20Metadata.sol";
import "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import "./ITerabethiaCore.sol";

contract ERC20Bridge {
    // Terabethia core contract.
    ITerabethiaCore terabethiaCore;

    // L2 Canister address
    uint256 constant CANISTER_ADDRESS = 0x00000000003001540101;

    /**
      Initializes the contract state.
    */
    constructor(ITerabethiaCore terabethiaCore_) {
        terabethiaCore = terabethiaCore_;
    }

    function withdraw(address token, uint256 amount) external {
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
    ) external {
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
}
