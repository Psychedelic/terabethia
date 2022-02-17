pragma solidity ^0.8.0;

import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "./ITerabethiaCore.sol";

contract ERC20Bridge {
    // Terabethia core contract.
    ITerabethiaCore terabethiaCore;

    // L2 Canister address
    uint256 constant CANISTER_ADDRESS = 0x00000000003001090101;

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
        IERC20(token).transfer(msg.sender, amount);
    }

    function deposit(
        address token,
        uint256 amount,
        uint256 user
    ) external payable {
        IERC20(token).transferFrom(msg.sender, address(this), amount);

        // Construct the deposit message's payload.
        uint256[] memory payload = new uint256[](3);
        payload[0] = uint256(uint160(token));
        payload[1] = user;
        payload[2] = amount;

        // Send the message to the IC
        terabethiaCore.sendMessage(CANISTER_ADDRESS, payload);
    }
}
