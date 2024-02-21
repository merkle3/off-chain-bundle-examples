// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.19;

import "forge-std/console.sol";
import "openzeppelin/access/Ownable.sol";
import "openzeppelin/token/ERC20/IERC20.sol";
import "./interfaces/IWETH.sol";
import "./interfaces/UniswapV2.sol";

// This contract is only callable by the deployer/owner, relying on internally held WETH balance
contract BlindBackrunExecutor is Ownable {
    uint256 constant uniswappyFee = 997;
    IWETH public immutable WETH;

    constructor(IWETH _wethAddress) Ownable(msg.sender) {
        WETH = _wethAddress;
    }

    /// @notice Execute an arbitrage transaction between two Uniswap V2 pairs.
    /// @notice Pair addresses need to be computed off-chain.
    /// @dev Only the contract owner can call this function.
    /// @param firstPairAddress Address of the first Uniswap V2 pair.
    /// @param secondPairAddress Address of the second Uniswap V2 pair.
    /// @param amountIn WETH amount to initiate the first swap
    /// @param firstPairDataIsWETHZero Bool whether pair is WETH/TOKEN or TOKEN/WETH
    /// @param firstPairAmountOut Amount expected to receive from first swap
    /// @param finalAmountOut Amount WETH after second swap
    /// @param profit Amount ETH total profit after both swaps
    /// @param profitToCoinbase Amount ETH to pay to block builder
    /// @param profitToCaller Amount ETH to withdraw to searcher as profit
    /// @param searcher Address of searcher to receive ETH profitToCaller
    function executeArbitrage(
        address firstPairAddress,
        address secondPairAddress,
        uint256 amountIn,
        bool firstPairDataIsWETHZero,
        uint256 firstPairAmountOut,
        uint256 finalAmountOut,
        uint256 profit,
        uint256 profitToCoinbase,
        uint256 profitToCaller,
        address searcher
    ) external onlyOwner {
        WETH.transfer(firstPairAddress, amountIn);

        IUniswapV2Pair firstPair = IUniswapV2Pair(firstPairAddress);
        IUniswapV2Pair secondPair = IUniswapV2Pair(secondPairAddress);

        console.log("\n--------- TRADING ---------");
        if (firstPairDataIsWETHZero == true) {
            firstPair.swap(0, firstPairAmountOut, secondPairAddress, "");
            console.log("firstPair swap done");
            
            secondPair.swap(finalAmountOut, 0, address(this), "");
            console.log("secondPair swap done");
        } else {
            firstPair.swap(firstPairAmountOut, 0, secondPairAddress, "");
            console.log("firstPair swap done");
            
            secondPair.swap(0, finalAmountOut, address(this), "");
            console.log("secondPair swap done");
        }
        
        WETH.withdraw(profit);
        block.coinbase.transfer(profitToCoinbase);
        payable(searcher).transfer(profitToCaller);
    }

    /// @notice Transfers all WETH held by the contract to the contract owner.
    /// @dev Only the contract owner can call this function.
    function withdrawWETHToOwner() external onlyOwner {
        uint256 balance = WETH.balanceOf(address(this));
        WETH.transfer(msg.sender, balance);
    }

    /// @notice Transfers all ETH held by the contract to the contract owner.
    /// @dev Only the contract owner can call this function.
    function withdrawETHToOwner() external onlyOwner {
        uint256 balance = address(this).balance;
        payable(msg.sender).transfer(balance);
    }
}
