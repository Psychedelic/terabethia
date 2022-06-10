pragma solidity ^0.8.0;

interface IWeth {
    /*
  unwrap weth to eth
  */
    function withdraw(uint256 amount) external;
}
