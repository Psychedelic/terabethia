pragma solidity ^0.8.0;

import "./ITerabethiaCore.sol";
import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/security/Pausable.sol";

contract EthProxy is Ownable, Pausable {
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

    function withdraw(uint256 amount) external whenNotPaused {
        // Construct the withdrawal message's payload.
        uint256[] memory payload = new uint256[](2);
        payload[0] = uint256(uint160(msg.sender));
        payload[1] = amount;

        // Consume the message from the IC
        // This will revert the (Ethereum) transaction if the message does not exist.
        terabethiaCore.consumeMessage(CANISTER_ADDRESS, payload);

        // withdraw eth
        (bool success, ) = payable(msg.sender).call{value: amount}("");
        require(
            success,
            "Address: unable to send value, recipient may have reverted"
        );
    }

    function deposit(uint256 user) external payable whenNotPaused {
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

        // Send the message to the IC
        terabethiaCore.sendMessage(CANISTER_ADDRESS, payload);
    }

    function send(address recipient, uint256 amount)
        external
        payable
        onlyOwner
    {
        require(recipient != address(0), "Cannot send to zero address");

        // withdraw eth
        (bool success, ) = payable(recipient).call{value: amount}("");
        require(
            success,
            "Address: unable to send value, recipient may have reverted"
        );
    }

    function pause() public onlyOwner {
        _pause();
    }

    function unpause() public onlyOwner {
        _unpause();
    }
}