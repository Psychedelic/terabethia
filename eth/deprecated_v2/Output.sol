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

library StarknetOutput {
    uint256 internal constant HEADER_SIZE = 0;

    // An event that is raised when a message is sent from L2 to L1.
    event LogMessageToL1(bytes32 indexed hash);

    // An event that is raised when a message from L1 to L2 is consumed.
    event ConsumedMessageToL2(bytes32 indexed hash);

    /**
      Does a sanity check of the output_data length.
    */
    function validate(bytes32[] calldata output_data) internal pure {
        require(output_data.length > HEADER_SIZE, "STARKNET_OUTPUT_TOO_SHORT");
    }

    /**
      Processes a message hashes received from the L2
      The 'messages' mapping is updated according to the messages and the direction ('isL2ToL1').
    */
    function processMessages(
        bool isL2ToL1,
        bytes32[] calldata hashes,
        mapping(bytes32 => uint256) storage messages
    ) internal returns (uint256) {
        // int256 tmp_message_segment_size = int256(programOutputSlice[0]);
        uint256 message_segment_size = uint256(hashes[0]);
        require(message_segment_size < 2**30, "INVALID_MESSAGE_SEGMENT_SIZE");

        uint256 offset = 1;
        uint256 message_segment_end = offset + message_segment_size;

        while (offset < message_segment_end) {
            // uint256 payloadLengthOffset = offset + MESSAGE_PAYLOAD_SIZE_OFFSET;
            require(offset <= hashes.length, "MESSAGE_TOO_SHORT");

            bytes32 messageHash = hashes[offset];

            if (isL2ToL1) {
                emit LogMessageToL1(messageHash);
                messages[messageHash] += 1;
            } else {
                require(
                    messages[messageHash] > 0,
                    "INVALID_MESSAGE_TO_CONSUME"
                );

                // Note that in the case of a message from L1 to L2, the selector (a single integer)
                // is prepended to the payload.
                emit ConsumedMessageToL2(messageHash);
                messages[messageHash] -= 1;
            }

            offset += 1;
        }

        require(offset == message_segment_end, "INVALID_MESSAGE_SEGMENT_SIZE");
        return offset;
    }
}
