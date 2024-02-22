// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.19;

/// @title BasicOnlySettle
/// @notice This contract provides basic functionalities for on-chain transactions.
contract BasicOnlySettle {

    address constant public merkleOffChainBundleSigner = 0x0000000000000000000000000000000000000000;

    /// @dev Emitted when a settlement event occurs.
    /// @param fees The fees associated with the settlement.
    /// @param _data Additional data related to the settlement.
    event OnChainEvent(uint256 indexed fees, bytes _data);

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
