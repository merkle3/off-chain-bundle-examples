// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.19;

/// @title Basic
/// @notice This contract provides basic functionalities for off-chain and on-chain transactions.
contract Basic {

    address constant public merkleOffChainBundleSigner = 0x0000000000000000000000000000000000000000;

    /// @dev Emitted when a settlement event occurs.
    /// @param fees The fees associated with the settlement.
    /// @param _data Additional data related to the settlement.
    event OnChainEvent(uint256 indexed fees, bytes _data);

    /// @dev Performs a search operation using the provided data.
    /// @param _fees The amount of fees to pay, passed through to the settle on-chain transaction call.
    /// @param _data The input data for the search operation.
    /// @return settleData The encoded calldata to call the `settle` function with fees and passthrough data.
    function search(uint256 _fees, bytes memory _data) external pure returns (bytes memory settleData) {

        // off-chain computation here using _data

        // return calldata that calls settle() with fees and passthrough data
        return abi.encode(this.settle.selector, _fees, _data);
    }

    /// @dev Performs a settlement operation.
    /// @param _fees The fees associated with the settlement.
    /// @param _data Additional data related to the settlement.
    function settle(uint256 _fees, bytes memory _data) external payable {
        require(msg.sender == merkleOffChainBundleSigner, "Caller must be a verified Merkle Off-Chain Bundle Signer");
        require(msg.value == block.number, "Hack to protect off-chain bundles from reorgs");

        emit OnChainEvent(_fees, _data);

        // on-chain computation here using _data

        // transfer fees to the coinbase address
        block.coinbase.transfer(_fees);
    }
}
