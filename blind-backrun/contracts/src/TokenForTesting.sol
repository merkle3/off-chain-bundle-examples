pragma solidity ^0.8.19;

import "openzeppelin/token/ERC20/ERC20.sol";

contract TokenForTesting is ERC20 {
    uint8 public TokenDecimals = 18;

    constructor(uint8 _decimals) ERC20("TokenForTesting", "TFT") {
        _mint(msg.sender, 1e22);
        TokenDecimals = _decimals;
    }

    function decimals() public view override returns (uint8) {
        return TokenDecimals;
    }
}
