pragma solidity ^0.8.0;

interface ITerabethiaCore {
    /**
      Sends a message to an L2 contract.

      Returns the hash of the message.
    */
    function sendMessage(uint256 to_address, uint256[] calldata payload)
        external
        returns (bytes32);

    /**
      Consumes a message that was sent from an L2 contract.

      Returns the hash of the message.
    */
    function consumeMessage(uint256 fromAddress, uint256[] calldata payload)
        external
        returns (bytes32);
}
