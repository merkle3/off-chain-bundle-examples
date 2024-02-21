// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.19;

import "openzeppelin/token/ERC20/IERC20.sol";
import "./BlindBackrunLogic.sol";
import "./interfaces/IWETH.sol";

// This contract is only callable by the deployer/owner, relying on internally held WETH balance
contract BlindBackrun is BlindBackrunLogic {

    constructor(IWETH _wethAddress) BlindBackrunLogic(_wethAddress) {}

    /// @notice Searches for an arbitrage transaction between two Uniswap V2 pairs.
    /// @notice Pair addresses need to be computed off-chain.
    /// @dev Only the contract owner can call this function.
    /// @param firstPairAddress Address of the first Uniswap V2 pair.
    /// @param secondPairAddress Address of the second Uniswap V2 pair.
    /// @param percentageToPayToCoinbase Amount of profit to pay to block builder
    /// @param searcher Address of searcher to withdraw profits
    /// @return optimal Struct containing complete optimal route, amounts, and profits
    function searchArbitrage(
        address firstPairAddress,
        address secondPairAddress,
        uint percentageToPayToCoinbase,
        address searcher
    ) external onlyOwner returns (bytes memory) {
        IOptimalArbitrage.OptimalArbitrage memory optimal = _searchArbitrage(
            firstPairAddress,
            secondPairAddress,
            percentageToPayToCoinbase,
            searcher
        );
        return abi.encodeWithSignature(
            "executeArbitrage(address,address,uint256,bool,uint256,uint256,uint256,uint256,uint256,address)",
            optimal.firstPairAddress,
            optimal.secondPairAddress,
            optimal.amountIn,
            optimal.firstPairDataIsWETHZero,
            optimal.firstPairAmountOut,
            optimal.finalAmountOut,
            optimal.profit,
            optimal.profitToCoinbase,
            optimal.profitToCaller,
            optimal.searcher
        );
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
        IOptimalArbitrage.OptimalArbitrage memory optimal = IOptimalArbitrage.OptimalArbitrage(
            firstPairAddress,
            secondPairAddress,
            amountIn,
            firstPairDataIsWETHZero,
            firstPairAmountOut,
            finalAmountOut,
            profit,
            profitToCoinbase,
            profitToCaller,
            searcher
        );
        _executeArbitrage(optimal);
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
