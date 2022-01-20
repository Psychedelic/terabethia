// SPDX-License-Identifier: GPL-3.0

pragma solidity >=0.8.0 <0.9.0;

import "hardhat/console.sol";

contract DebugContract {
    function split(bytes32 word) public returns (uint256) {
        uint256 msgInt = uint256(word);
        uint256 a = uint256(uint128(msgInt));
        uint256 b = uint256(uint128(msgInt >> 128));
        console.log(a);
        console.log(b);
        return a;
    }
}
