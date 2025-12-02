// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "./PositionUtilsPCSV4.sol";

/// @notice A lens for PancakeSwap Infinity that peeks into the current state of position and pool info without deployment
/// @author Aperture Finance
/// @dev The return data can be accessed externally by `eth_call` without a `to` address or internally by catching the
/// revert data, and decoded by `abi.decode(data, (PositionState))`
contract EphemeralGetPositionPCSV4 is PositionUtilsPCSV4 {
    constructor(address positionManagerAddr, uint256 tokenId) payable {
        PositionState memory pos = getPosition(positionManagerAddr, tokenId);
        bytes memory returnData = abi.encode(pos);
        assembly ("memory-safe") {
            revert(add(returnData, 0x20), mload(returnData))
        }
    }

    /// @dev Public function to expose the abi for easier decoding using TypeChain
    /// @param positionManagerAddr PancakeSwap Infinity's position manager contract address
    /// @param tokenId Token ID of the position
    function getPosition(address positionManagerAddr, uint256 tokenId)
        public
        payable
        returns (PositionState memory state)
    {
        positionInPlace(positionManagerAddr, tokenId, state);
        peek(positionManagerAddr, tokenId, state);
    }
}
