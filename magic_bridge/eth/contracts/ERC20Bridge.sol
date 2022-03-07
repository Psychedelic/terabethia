pragma solidity ^0.8.0;

import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/token/ERC20/extensions/IERC20Metadata.sol";
import "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import "./ITerabethiaCore.sol";

function strToUint(string memory _str) returns(uint256 res, bool err) {
    bytes memory b = bytes(_str);

    for (uint256 i = 0; i < b.length; i++) {
        if ((uint8(b[i]) - 48) < 0 || (uint8(b[i]) - 48) > 9) {
            return (0, true);
        }
        res += (uint8(b[i]) - 48) * 10**(b.length - i - 1);
    }
    
    return (res, false);
}


contract ERC20Bridge {
    // Terabethia core contract.
    ITerabethiaCore terabethiaCore;

    // L2 Canister address
    uint256 constant CANISTER_ADDRESS = 0x000000000030010a0101;

    /**
      Initializes the contract state.
    */
    constructor(ITerabethiaCore terabethiaCore_) {
        terabethiaCore = terabethiaCore_;
    }

    function withdraw(address token, uint256 amount) external {
        // Construct the withdrawal message's payload.
        uint256[] memory payload = new uint256[](3);
        payload[0] = uint256(uint160(token));
        payload[1] = uint256(uint160(msg.sender));
        payload[2] = amount;

        // Consume the message from the IC
        // This will revert the (Ethereum) transaction if the message does not exist.
        terabethiaCore.consumeMessage(CANISTER_ADDRESS, payload);

        // withdraw erc20
        SafeERC20.safeTransfer(IERC20(token), msg.sender, amount);
    }

    function deposit(
        address token,
        uint256 amount,
        uint256 user
    ) external payable {
        // Convert token name to uint256
        (uint256 tokenName, bool tokenNameHasError) = strToUint(IERC20Metadata(token).name());
        require(tokenNameHasError == false, "Unable to convert token name to uint256");

        // Convert token symbol to uint256
        (uint256 tokenSymbol, bool tokenSymbolHasError) = strToUint(IERC20Metadata(token).symbol());
        require(tokenSymbolHasError == false, "Unable to convert token symbol to uint256");

        SafeERC20.safeTransferFrom(IERC20(token), msg.sender, address(this), amount);

        // Construct the deposit message's payload.
        uint256[] memory payload = new uint256[](6);
        payload[0] = uint256(uint160(token));
        payload[1] = user;
        payload[2] = amount;
        payload[3] = tokenName;
        payload[4] = tokenSymbol;
        payload[5] = uint256(IERC20Metadata(token).decimals());

        // Send the message to the IC
        terabethiaCore.sendMessage(CANISTER_ADDRESS, payload);
    }
}
