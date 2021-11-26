pragma solidity ^0.6.12;

import "hardhat/console.sol";
import "./IStarknetMessaging.sol";

contract EthProxy {
    // The StarkNet core contract.
    IStarknetMessaging starknetCore;

    uint256 constant MESSAGE_WITHDRAW = 0;

    // The selector of the "deposit" l1_handler.
    uint256 constant CANISTER_ADDRESS = 0x00000000003000ea0101;

    /**
      Initializes the contract state.
    */
    constructor(IStarknetMessaging starknetCore_) public {
        starknetCore = starknetCore_;
    }

    function withdraw(uint256 amount) external {
        // Construct the withdrawal message's payload.
        uint256[] memory payload = new uint256[](3);
        payload[0] = MESSAGE_WITHDRAW;
        payload[1] = uint256(uint160(msg.sender));
        payload[2] = amount;

        // Consume the message from the StarkNet core contract.
        // This will revert the (Ethereum) transaction if the message does not exist.
        starknetCore.consumeMessageFromL2(CANISTER_ADDRESS, payload);

        // withdraw eth
        require(
            address(this).balance >= amount,
            "Address: insufficient balance"
        );

        (bool success, ) = payable(msg.sender).call{value: amount}("");
        require(
            success,
            "Address: unable to send value, recipient may have reverted"
        );
    }

    function deposit(uint256 user) external payable {
        require(msg.value >= 1 gwei, "DepositContract: deposit value too low");
        require(
            msg.value % 1 gwei == 0,
            "DepositContract: deposit value not multiple of gwei"
        );

        uint256 deposit_amount = msg.value / 1 gwei;

        require(
            deposit_amount <= type(uint64).max,
            "DepositContract: deposit value too high"
        );

        // Construct the deposit message's payload.
        uint256[] memory payload = new uint256[](2);
        payload[0] = user;
        payload[1] = deposit_amount;

        // Send the message to the StarkNet core contract.
        starknetCore.sendMessageToL2(CANISTER_ADDRESS, payload);
    }
}
