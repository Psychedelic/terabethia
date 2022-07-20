pragma solidity ^0.8.0;

interface IEthProxy {
    /*
    unwrap weth to eth
    */
    function deposit(uint256 amount) external payable;
}
