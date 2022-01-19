// SPDX-License-Identifier: Unlicensed
pragma solidity ^0.6.12;

import "./Output.sol";

library TerabethiaState {
    struct State {
        // bytes32 globalRoot;
        int256 sequenceNumber;
    }

    function copy(State storage state, State memory stateFrom) internal {
        state.sequenceNumber = stateFrom.sequenceNumber;
    }

    /**
      Validates that the 'sequenceNumber'
    */
    function update(State storage state, int256 sequenceNumber) internal {
        // Check the sequenceNumber first as the error is less ambiguous then INVALID_PREVIOUS_ROOT.
        state.sequenceNumber += 1;
        require(
            state.sequenceNumber == sequenceNumber,
            "INVALID_SEQUENCE_NUMBER"
        );
    }
}
