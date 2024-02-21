// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.19;

import "forge-std/console.sol";
import "openzeppelin/access/Ownable.sol";
import "./interfaces/IWETH.sol";
import "./interfaces/UniswapV2.sol";

interface IPairReserves {
    struct PairReserves {
        uint256 reserve0;
        uint256 reserve1;
        uint256 price;
        bool isWETHZero;
    }
}

interface IOptimalArbitrage {
    struct OptimalArbitrage {
        address firstPairAddress;
        address secondPairAddress;
        uint256 amountIn;
        bool firstPairDataIsWETHZero;
        uint256 firstPairAmountOut;
        uint256 finalAmountOut;
        uint256 profit;
        uint256 profitToCoinbase;
        uint256 profitToCaller;
        address searcher;
    }
}

// Do not simply open up _executeArbitrage to any caller, this code should either be protected by onlyOwner or should not maintain a WETH balance
abstract contract BlindBackrunLogic is Ownable {
    uint256 constant uniswappyFee = 997;
    IWETH public immutable WETH;

    constructor(IWETH _wethAddress) Ownable(msg.sender) {
        WETH = _wethAddress;
    }

    /// @notice Searches for an arbitrage transaction between two Uniswap V2 pairs.
    /// @notice Pair addresses need to be computed off-chain.
    /// @dev Only the contract owner can call this function.
    /// @param firstPairAddress Address of the first Uniswap V2 pair.
    /// @param secondPairAddress Address of the second Uniswap V2 pair.
    /// @param percentageToPayToCoinbase Amount of profit to pay to block builder
    /// @param searcher Address of searcher to withdraw profits
    /// @return optimal Struct containing complete optimal route, amounts, and profits
    function _searchArbitrage(
        address firstPairAddress,
        address secondPairAddress,
        uint percentageToPayToCoinbase,
        address searcher
    ) internal returns (IOptimalArbitrage.OptimalArbitrage memory optimal) {
        uint256 balanceBefore = WETH.balanceOf(address(this));
        console.log("Starting balance  : %s", balanceBefore);
        IUniswapV2Pair firstPair = IUniswapV2Pair(firstPairAddress);
        IUniswapV2Pair secondPair = IUniswapV2Pair(secondPairAddress);

        console.log("\n--------- PAIR DATA ---------");

        IPairReserves.PairReserves memory firstPairData = getPairData(firstPair);
        IPairReserves.PairReserves memory secondPairData = getPairData(secondPair);

        console.log("\n--------- PRICES ---------");
        console.log("firstPair price   : %s", firstPairData.price);
        console.log("secondPair price  : %s", secondPairData.price);

        uint256 amountIn = getAmountIn(firstPairData, secondPairData);

        WETH.transfer(firstPairAddress, amountIn);

        bool firstPairDataIsWETHZero = firstPairData.isWETHZero;
        uint256 firstPairAmountOut;
        uint256 finalAmountOut;
        if (firstPairDataIsWETHZero == true) {
            firstPairAmountOut = getAmountOut(amountIn, firstPairData.reserve0, firstPairData.reserve1);
            finalAmountOut = getAmountOut(firstPairAmountOut, secondPairData.reserve1, secondPairData.reserve0);

            console.log("\n--------- TRADE AMOUNTS ---------");
            console.log("amountIn          : %s", amountIn);
            console.log("firstPairAmountOut: %s", firstPairAmountOut);
            console.log("finalAmountOut    : %s", finalAmountOut);
            console.log("arb profit        : %s", finalAmountOut - amountIn);
            
            firstPair.swap(0, firstPairAmountOut, secondPairAddress, "");
            console.log("firstPair swap done");
            
            secondPair.swap(finalAmountOut, 0, address(this), "");
            console.log("secondPair swap done");
        } else {
            firstPairAmountOut = getAmountOut(amountIn, firstPairData.reserve1, firstPairData.reserve0);
            finalAmountOut = getAmountOut(firstPairAmountOut, secondPairData.reserve0, secondPairData.reserve1);

            console.log("\n--------- TRADE AMOUNTS ---------");
            console.log("amountIn          : %s", amountIn);
            console.log("firstPairAmountOut: %s", firstPairAmountOut);
            console.log("finalAmountOut    : %s", finalAmountOut);
            console.log("arb profit        : %s", finalAmountOut - amountIn);
            
            firstPair.swap(firstPairAmountOut, 0, secondPairAddress, "");
            console.log("firstPair swap done");
            
            secondPair.swap(0, finalAmountOut, address(this), "");
            console.log("secondPair swap done");
        }
        
        uint256 balanceAfter = WETH.balanceOf(address(this));
        require(balanceAfter > balanceBefore, "Arbitrage failed");

        uint profit = balanceAfter - balanceBefore;
        console.log("\n--------- SUCCESS ---------");
        console.log("Ending balance    : %s", balanceAfter);
        console.log("Profit            : %s", profit);
        WETH.withdraw(profit);

        uint profitToCoinbase = profit * percentageToPayToCoinbase / 100;
        uint profitToCaller = profit - profitToCoinbase;
        block.coinbase.transfer(profitToCoinbase);
        payable(searcher).transfer(profitToCaller);

        optimal.firstPairAddress = firstPairAddress;
        optimal.secondPairAddress = secondPairAddress;
        optimal.amountIn = amountIn;
        optimal.firstPairDataIsWETHZero = firstPairDataIsWETHZero;
        optimal.firstPairAmountOut = firstPairAmountOut;
        optimal.finalAmountOut = finalAmountOut;
        optimal.profitToCoinbase = profitToCoinbase;
        optimal.profitToCaller = profitToCaller;
        optimal.searcher = searcher;
    }

    /// @notice Execute an arbitrage transaction between two Uniswap V2 pairs.
    /// @notice Pair addresses need to be computed off-chain.
    /// @dev Only the contract owner can call this function.
    /// @param optimal Struct containing complete optimal route, amounts, and profits
    function _executeArbitrage(IOptimalArbitrage.OptimalArbitrage memory optimal) internal {
        WETH.transfer(optimal.firstPairAddress, optimal.amountIn);

        IUniswapV2Pair firstPair = IUniswapV2Pair(optimal.firstPairAddress);
        IUniswapV2Pair secondPair = IUniswapV2Pair(optimal.secondPairAddress);

        console.log("\n--------- TRADING ---------");
        if (optimal.firstPairDataIsWETHZero == true) {
            firstPair.swap(0, optimal.firstPairAmountOut, optimal.secondPairAddress, "");
            console.log("firstPair swap done");
            
            secondPair.swap(optimal.finalAmountOut, 0, address(this), "");
            console.log("secondPair swap done");
        } else {
            firstPair.swap(optimal.firstPairAmountOut, 0, optimal.secondPairAddress, "");
            console.log("firstPair swap done");
            
            secondPair.swap(0, optimal.finalAmountOut, address(this), "");
            console.log("secondPair swap done");
        }
        
        WETH.withdraw(optimal.profit);
        block.coinbase.transfer(optimal.profitToCoinbase);
        payable(optimal.searcher).transfer(optimal.profitToCaller);
    }

    /// @notice Calculates the required input amount for the arbitrage transaction.
    /// @param firstPairData Struct containing data about the first Uniswap V2 pair.
    /// @param secondPairData Struct containing data about the second Uniswap V2 pair.
    /// @return amountIn, the optimal amount to trade to arbitrage two v2 pairs.
    function getAmountIn(
        IPairReserves.PairReserves memory firstPairData, 
        IPairReserves.PairReserves memory secondPairData
    ) public view returns (uint256) {
        uint256 numerator = getNumerator(firstPairData, secondPairData);
        console.log("numerator: %s", numerator);
        uint256 denominator = getDenominator(firstPairData, secondPairData);
        console.log("denominator: %s", denominator);
        uint256 amountIn = 
            numerator
            * 1000
            / denominator;

        return amountIn;
    }

    function getNumerator(
        IPairReserves.PairReserves memory firstPairData, 
        IPairReserves.PairReserves memory secondPairData
    ) public view returns (uint256) {
        if (firstPairData.isWETHZero == true) {
            uint presqrt = uniswappyFee
                    * uniswappyFee
                    * firstPairData.reserve1
                    * secondPairData.reserve0
                    / secondPairData.reserve1
                    / firstPairData.reserve0;
            console.log("presqrt: %s", presqrt);
            console.log("sqrt(presqrt): %s", sqrt(presqrt));
            uint256 numerator = 
            (
                sqrt(presqrt)
                - 1e3
            )            
            * secondPairData.reserve1
            * firstPairData.reserve0;

            return numerator;
        } else {
            uint presqrt = uniswappyFee
                * uniswappyFee
                * firstPairData.reserve0
                * secondPairData.reserve1
                / secondPairData.reserve0
                / firstPairData.reserve1;


            uint256 numerator = 
            (
                sqrt(presqrt)
                - 1e3
            )            
            * secondPairData.reserve0
            * firstPairData.reserve1;

            return numerator;
        }
    }

    function getDenominator(
            IPairReserves.PairReserves memory firstPairData, 
            IPairReserves.PairReserves memory secondPairData
        ) public pure returns (uint256){
        if (firstPairData.isWETHZero == true) {
            uint256 denominator = 
                (
                    uniswappyFee
                    * secondPairData.reserve1
                    * 1000
                )
                + (
                    uniswappyFee
                    * uniswappyFee
                    * firstPairData.reserve1
                );
            return denominator;
        } else {
            uint256 denominator = 
                (
                    uniswappyFee
                    * secondPairData.reserve0
                    * 1000
                )
                + (
                    uniswappyFee
                    * uniswappyFee
                    * firstPairData.reserve0
                );
            return denominator;
        }
    }

    /// @notice Retrieves price and reserve data for a given Uniswap V2 pair. Also checks which token is WETH.
    /// @param pair The Uniswap V2 pair to retrieve data for.
    /// @return A IPairReserves.PairReserves struct containing price and reserve data for the given pair.
    function getPairData(IUniswapV2Pair pair) private view returns (IPairReserves.PairReserves memory) {
        (uint256 reserve0, uint256 reserve1, ) = pair.getReserves();
        console.log("pair:", address(pair));
        console.log("reserve0:",reserve0);
        console.log("reserve1:", reserve1);

        uint256 price;

        bool isWETHZero = false;
        if (pair.token0() == address(WETH)) {
            price = reserve1 * 1e18 / reserve0;
            isWETHZero = true;
        } else {
            price = reserve0 * 1e18 / reserve1;
        }

        return IPairReserves.PairReserves(reserve0, reserve1, price, isWETHZero);
    }

    /// @notice Calculates the square root of a given number.
    /// @param x The number to calculate the square root of.
    /// @return y, The square root of the given number.
    function sqrt(uint256 x) private pure returns (uint256) {
        if (x == 0) return 0;
        uint256 z = (x + 1) / 2;
        uint256 y = x;
        while (z < y) {
            y = z;
            z = ((x / z) + z) / 2;
        }
        return y;
    }

    /// @notice Calculates the output amount for a given input amount and reserves.
    /// @param amountIn The input amount.
    /// @param reserveIn The reserve of the input token.
    /// @param reserveOut The reserve of the output token.
    /// @return amountOut The output amount.
    function getAmountOut(uint amountIn,
        uint reserveIn,
        uint reserveOut
    ) internal pure returns (uint amountOut) {
        uint amountInWithFee = amountIn * 997;
        uint numerator = amountInWithFee * reserveOut;
        uint denominator = reserveIn * 1000 + amountInWithFee;
        amountOut = numerator / denominator;
        return amountOut;
    }

    /// @notice Executes a call to another contract with the provided data and value. The owner of this contract can help rescue assets accidentally left on contract, but in normal operation, this contract does not hold assets and is not centrally controlled.
    /// @dev Only the contract owner can call this function.
    /// @dev Reverted calls will result in a revert. 
    /// @param _to The address of the contract to call.
    /// @param _value The amount of Ether to send with the call.
    /// @param _data The calldata to send with the call.
    function call(address payable _to, uint256 _value, bytes memory _data) external onlyOwner {
        (bool success, ) = _to.call{value: _value}(_data);
        require(success, "External call failed");
    }

    /// @notice Fallback function that allows the contract to receive Ether.
    receive() external payable {}
}
