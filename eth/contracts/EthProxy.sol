pragma solidity ^0.6.12;

import "./IStarknetMessaging.sol";

contract EthProxy {
    // The StarkNet core contract.
    IStarknetMessaging starknetCore;

    mapping(bytes32 => uint256) public userBalances;

    uint256 constant MESSAGE_WITHDRAW = 0;

    // The selector of the "deposit" l1_handler.
    bytes32 constant CANISTER_ADDRESS =
        0x6d6e6932637a71616161616161616471616c3671636169000000000000000000;

    /**
      Initializes the contract state.
    */
    constructor(IStarknetMessaging starknetCore_) public {
        starknetCore = starknetCore_;
    }

    function withdraw(uint256 amount) external {
        // Construct the withdrawal message's payload.
        bytes32[] memory payload = new bytes32[](3);
        payload[0] = MESSAGE_WITHDRAW;
        payload[1] = bytes32(uint256(uint160(msg.sender)));
        payload[2] = bytes32(amount);

        // Consume the message from the StarkNet core contract.
        // This will revert the (Ethereum) transaction if the message does not exist.
        starknetCore.consumeMessageFromL2(CANISTER_ADDRESS, payload);

        // Update the L1 balance.
        userBalances[user] += amount;
    }

    function deposit(bytes32 user, uint256 amount) external {
        require(amount < 2**64, "Invalid amount.");
        require(
            amount <= userBalances[user],
            "The user's balance is not large enough."
        );

        // Update the L1 balance.
        userBalances[user] -= amount;

        // Construct the deposit message's payload.
        bytes32[] memory payload = new bytes32[](2);
        payload[0] = user;
        payload[1] = bytes32(amount);

        // Send the message to the StarkNet core contract.
        starknetCore.sendMessageToL2(CANISTER_ADDRESS, payload);
    }

    function balanceOf(bytes32 account)
        public
        view
        virtual
        override
        returns (uint256)
    {
        return userBalances[account];
    }
}
