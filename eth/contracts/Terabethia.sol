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
pragma experimental ABIEncoderV2;

// import "./IFactRegistry.sol";
import "./IIdentity.sol";
import "./Output.sol";
import "./StarknetGovernance.sol";
import "./StarknetMessaging.sol";
import "./StarknetOperator.sol";
import "./NamedStorage.sol";
import "./ContractInitializer.sol";
import "./ProxySupport.sol";

import "./TerabethiaState.sol";

contract Terabethia is
    IIdentity,
    StarknetGovernance,
    StarknetMessaging,
    StarknetOperator,
    ContractInitializer,
    ProxySupport
{
    using TerabethiaState for TerabethiaState.State;

    // Logs the new state following a state update.
    event LogStateUpdate(int256 sequenceNumber);

    string internal constant STATE_STRUCT_TAG = "TERABETHIA_1.0_STATE_STRUCT";

    // State variable "state" access functions.
    function state()
        internal
        pure
        returns (TerabethiaState.State storage stateStruct)
    {
        bytes32 location = keccak256(abi.encodePacked(STATE_STRUCT_TAG));
        assembly {
            stateStruct_slot := location
        }
    }

    function isInitialized() internal view override returns (bool) {
        return state().sequenceNumber > 0;
    }

    function validateInitData(bytes calldata data) internal pure override {
        require(data.length == 32, "ILLEGAL_INIT_DATA_SIZE");
    }

    function initializeContractState(bytes calldata data) internal override {
        TerabethiaState.State memory initialState = abi.decode(
            data,
            (TerabethiaState.State)
        );

        state().copy(initialState);
        initGovernance();
    }

    /**
      Returns a string that identifies the contract.
    */
    function identify() external pure override returns (string memory) {
        return "Terabethia_2021_1";
    }

    /**
      Returns the current state root.
    */
    // function stateRoot() external view returns (bytes32) {
    //     return state().globalRoot;
    // }

    /**
      Returns the current sequence number.
    */
    function stateSequenceNumber() external view returns (int256) {
        return state().sequenceNumber;
    }

    /**
      Updates the state of the StarkNet, based on a proof of the 
      StarkNet OS that the state transition is valid.

      Arguments:
        sequenceNumber - The expected sequence number of the new block.
        programOutput - The main part of the StarkNet OS program output.
        data_availability_fact - An encoding of the on-chain data associated
        with the 'programOutput'.

        OnchainDataFactTreeEncoder.DataAvailabilityFact
        calldata data_availability_fact
    */
    function updateState(int256 sequenceNumber, bytes32[] calldata output)
        external
        onlyOperator
    {
        // Validate program output.
        StarknetOutput.validate(output);

        // Process L2 -> L1 messages.
        uint256 outputOffset = 0;
        outputOffset += StarknetOutput.processMessages(
            // isL2ToL1=
            true,
            output[outputOffset:],
            l2ToL1Messages()
        );

        // Process L1 -> L2 messages.
        outputOffset += StarknetOutput.processMessages(
            // isL2ToL1=
            false,
            output[outputOffset:],
            l1ToL2Messages()
        );

        require(outputOffset == output.length, "STARKNET_OUTPUT_TOO_LONG");

        // Perform state update.
        state().update(sequenceNumber);
        TerabethiaState.State memory state_ = state();
        emit LogStateUpdate(state_.sequenceNumber);
    }
}
