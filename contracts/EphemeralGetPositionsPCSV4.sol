// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "./PositionUtilsPCSV4.sol";

/// @notice A lens for PancakeSwap Infinity that peeks into the current state of positions and pool info without deployment
/// @author Aperture Finance
/// @dev The return data can be accessed externally by `eth_call` without a `to` address or internally by catching the
/// revert data, and decoded by `abi.decode(data, (PositionState[]))`
contract EphemeralGetPositionsPCSV4 is PositionUtilsPCSV4 {
    constructor(address positionManagerAddr, uint256[] memory tokenIds) payable {
        PositionState[] memory positions = getPositions(positionManagerAddr, tokenIds);
        bytes memory returnData = abi.encode(positions);
        assembly ("memory-safe") {
            revert(add(returnData, 0x20), mload(returnData))
        }
    }

    /// @dev Public function to expose the abi for easier decoding using TypeChain
    /// @param positionManagerAddr The address of PCS Infinity position manager
    /// @param tokenIds Token IDs of the positions
    function getPositions(address positionManagerAddr, uint256[] memory tokenIds)
        public
        payable
        returns (PositionState[] memory positions)
    {
        uint256 length = tokenIds.length;
        positions = new PositionState[](length);
        uint256 i;
        for (uint256 j; j < length; ++j) {
            uint256 tokenId = tokenIds[j];
            PositionState memory state = positions[i];
            if (positionInPlace(positionManagerAddr, tokenId, state)) {
                ++i;
                peek(positionManagerAddr, tokenId, state);
            }
        }
    }
}
