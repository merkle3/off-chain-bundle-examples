// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.19;

/// @title BasicOnlySearch
/// @notice This contract provides basic functionalities for off-chain transactions.
contract BasicOnlySearch {

    /// @dev Performs a search operation using the provided data.
    /// @param _data The input data for the search operation.
    /// @return settleData The encoded calldata to call the `settle` function with fees and passthrough data.
    function search(bytes memory _data) external pure returns (bytes memory settleData) {
        (uint256 _fees, bytes memory _settleData) = abi.decode(_data, (uint256, bytes));

        // off-chain computation here using _data

        // return calldata that calls settle() with fees and passthrough data
        return abi.encode("settle(uint256,bytes)", _fees, _settleData);
    }
}
