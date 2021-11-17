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
// SPDX-License-Identifier: Apache-2.0.
pragma solidity ^0.6.12;

library CommitmentTreeUpdateOutput {
    /**
      Returns the previous commitment tree root.
    */
    function getPrevRoot(uint256[] calldata commitment_tree_update_data)
        internal
        pure
        returns (uint256)
    {
        return commitment_tree_update_data[0];
    }

    /**
      Returns the new commitment tree root.
    */
    function getNewRoot(uint256[] calldata commitment_tree_update_data)
        internal
        pure
        returns (uint256)
    {
        return commitment_tree_update_data[1];
    }
}

library StarknetOutput {
    uint256 internal constant MERKLE_UPDATE_OFFSET = 0;
    uint256 internal constant HEADER_SIZE = 2;

    uint256 constant MESSAGE_FROM_ADDRESS_OFFSET = 0;
    uint256 constant MESSAGE_TO_ADDRESS_OFFSET = 1;
    uint256 constant MESSAGE_PAYLOAD_SIZE_OFFSET = 2;
    uint256 constant MESSAGE_PREFIX_SIZE = 3;
    uint256 constant SELECTOR_SIZE = 1;

    // An event that is raised when a message is sent from L2 to L1.
    event LogMessageToL1(
        uint256 indexed from_address,
        address indexed to_address,
        uint256[] payload
    );

    // An event that is raised when a message from L1 to L2 is consumed.
    event ConsumedMessageToL2(
        address indexed from_address,
        uint256 indexed to_address,
        uint256 indexed selector,
        uint256[] payload
    );

    /**
      Does a sanity check of the output_data length.
    */
    function validate(uint256[] calldata output_data) internal pure {
        require(output_data.length > HEADER_SIZE, "STARKNET_OUTPUT_TOO_SHORT");
    }

    /**
      Returns a slice of the 'output_data' with the commitment tree update information.
    */
    function getMerkleUpdate(uint256[] calldata output_data)
        internal
        pure
        returns (uint256[] calldata)
    {
        return output_data[MERKLE_UPDATE_OFFSET:MERKLE_UPDATE_OFFSET + 2];
    }

    /**
      Processes a message segment from the program output.
      The format of a message segment is the length of the messages in words followed
      by the concatenation of all the messages.

      The 'messages' mapping is updated according to the messages and the direction ('isL2ToL1').
    */
    function processMessages(
        bool isL2ToL1,
        uint256[] calldata programOutputSlice,
        mapping(bytes32 => uint256) storage messages
    ) internal returns (uint256) {
        uint256 message_segment_size = programOutputSlice[0];
        require(message_segment_size < 2**30, "INVALID_MESSAGE_SEGMENT_SIZE");

        uint256 offset = 1;
        uint256 message_segment_end = offset + message_segment_size;
        while (offset < message_segment_end) {
            uint256 payloadLengthOffset = offset + MESSAGE_PAYLOAD_SIZE_OFFSET;
            require(
                payloadLengthOffset < programOutputSlice.length,
                "MESSAGE_TOO_SHORT"
            );

            uint256 payloadLength = programOutputSlice[payloadLengthOffset];
            require(payloadLength < 2**30, "INVALID_PAYLOAD_LENGTH");

            uint256 endOffset = offset + MESSAGE_PREFIX_SIZE + payloadLength;
            require(
                endOffset <= programOutputSlice.length,
                "TRUNCATED_MESSAGE_PAYLOAD"
            );

            bytes32 messageHash = keccak256(
                abi.encodePacked(programOutputSlice[offset:endOffset])
            );
            if (isL2ToL1) {
                emit LogMessageToL1(
                    // from=
                    programOutputSlice[offset + MESSAGE_FROM_ADDRESS_OFFSET],
                    // to=
                    address(
                        programOutputSlice[offset + MESSAGE_TO_ADDRESS_OFFSET]
                    ),
                    // payload=
                    (uint256[])(
                        programOutputSlice[offset +
                            MESSAGE_PREFIX_SIZE:endOffset]
                    )
                );
                messages[messageHash] += 1;
            } else {
                require(
                    messages[messageHash] > 0,
                    "INVALID_MESSAGE_TO_CONSUME"
                );

                // Note that in the case of a message from L1 to L2, the selector (a single integer)
                // is prepended to the payload.
                emit ConsumedMessageToL2(
                    // from=
                    address(
                        programOutputSlice[offset + MESSAGE_FROM_ADDRESS_OFFSET]
                    ),
                    // to=
                    programOutputSlice[offset + MESSAGE_TO_ADDRESS_OFFSET],
                    // selector=
                    programOutputSlice[offset + MESSAGE_PREFIX_SIZE],
                    // payload=
                    (uint256[])(
                        programOutputSlice[offset +
                            MESSAGE_PREFIX_SIZE +
                            1:endOffset]
                    )
                );
                messages[messageHash] -= 1;
            }

            offset = endOffset;
        }
        require(offset == message_segment_end, "INVALID_MESSAGE_SEGMENT_SIZE");

        return offset;
    }
}
