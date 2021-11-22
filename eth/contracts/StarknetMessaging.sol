/*
  Copyright 2019-2021 StarkWare Industries Ltd.

  Licensed under the Apache License, Version 2.0 (the "License").
  You may not use this file except in compliance with the License.
  You may obtain a copy of the License at

  https://www.starkware.co/open-source-license/

  Unless required by applicable law or agreed to in writing,
  software distributed under the License is distributed on an "AS IS" BASIS,
  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
  See the License for the specific language governing permissions
  and limitations under the License.
*/
pragma solidity ^0.6.12;

import "./IStarknetMessaging.sol";
import "./NamedStorage.sol";

/**
  Implements sending messages to L2 by adding them to a pipe and consuming messages from L2 by
  removing them from a different pipe. A deriving contract can handle the former pipe and add items
  to the latter pipe while interacting with L2.
*/
contract StarknetMessaging is IStarknetMessaging {
    /**
      Random slot storage elements and accessors.
    */
    string constant L1L2_MESSAGE_MAP_TAG =
        "TERABETHIA_1.0_MSGING_L1TOL2_MAPPPING";
    string constant L2L1_MESSAGE_MAP_TAG =
        "TERABETHIA_1.0_MSGING_L2TOL1_MAPPPING";

    function l1ToL2Messages(bytes32 msgHash) external view returns (uint256) {
        return l1ToL2Messages()[msgHash];
    }

    function l2ToL1Messages(bytes32 msgHash) external view returns (uint256) {
        return l2ToL1Messages()[msgHash];
    }

    function l1ToL2Messages()
        internal
        pure
        returns (mapping(bytes32 => uint256) storage)
    {
        return NamedStorage.bytes32ToUint256Mapping(L1L2_MESSAGE_MAP_TAG);
    }

    function l2ToL1Messages()
        internal
        pure
        returns (mapping(bytes32 => uint256) storage)
    {
        return NamedStorage.bytes32ToUint256Mapping(L2L1_MESSAGE_MAP_TAG);
    }

    /**
      Sends a message to an L2 contract.
    */
    function sendMessageToL2(bytes32 to_address, bytes32[] calldata payload)
        external
        override
        returns (bytes32)
    {
        emit LogMessageToL2(msg.sender, to_address, payload);
        // Note that the selector (a single integer) is prepended to the payload.
        bytes32 msgHash = keccak256(
            abi.encodePacked(
                uint256(msg.sender),
                uint256(to_address),
                payload.length,
                payload
            )
        );
        l1ToL2Messages()[msgHash] += 1;
        return msgHash;
    }

    /**
      Consumes a message that was sent from an L2 contract.

      Returns the hash of the message.
    */
    function consumeMessageFromL2(
        bytes32 from_address,
        bytes32[] calldata payload
    ) external override returns (bytes32) {
        bytes32 msgHash = keccak256(
            abi.encodePacked(
                bytes32(from_address),
                msg.sender,
                uint256(payload.length),
                payload
            )
        );

        require(l2ToL1Messages()[msgHash] > 0, "INVALID_MESSAGE_TO_CONSUME");
        emit ConsumedMessageToL1(from_address, msg.sender, payload);
        l2ToL1Messages()[msgHash] -= 1;
        return msgHash;
    }
}
