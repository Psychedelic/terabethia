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

import "./Output.sol";

library StarknetState {
    struct State {
        uint256 globalRoot;
        int256 sequenceNumber;
    }

    function copy(State storage state, State memory stateFrom) internal {
        state.globalRoot = stateFrom.globalRoot;
        state.sequenceNumber = stateFrom.sequenceNumber;
    }

    /**
      Validates that the 'sequenceNumber' and the previous root are consistent with the
      current state and updates the state.
    */
    function update(
        State storage state,
        int256 sequenceNumber,
        uint256[] calldata starknetOutput
    ) internal {
        // Check the sequenceNumber first as the error is less ambiguous then INVALID_PREVIOUS_ROOT.
        state.sequenceNumber += 1;
        require(
            state.sequenceNumber == sequenceNumber,
            "INVALID_SEQUENCE_NUMBER"
        );

        uint256[] calldata commitment_tree_update = StarknetOutput
            .getMerkleUpdate(starknetOutput);
        require(
            state.globalRoot ==
                CommitmentTreeUpdateOutput.getPrevRoot(commitment_tree_update),
            "INVALID_PREVIOUS_ROOT"
        );
        state.globalRoot = CommitmentTreeUpdateOutput.getNewRoot(
            commitment_tree_update
        );
    }
}
