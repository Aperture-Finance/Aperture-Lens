// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import {NPMCaller} from "@aperture_finance/uni-v3-lib/src/NPMCaller.sol";
import "./PositionUtils.sol";

/// @notice A lens for Uniswap v3 that peeks into the current state of positions and pool info without deployment
/// @author Aperture Finance
/// @dev The return data can be accessed externally by `eth_call` without a `to` address or internally by catching the
/// revert data, and decoded by `abi.decode(data, (PositionState[]))`
contract EphemeralGetPositions is PositionUtils {
    constructor(DEX dex, address npm, uint256[] memory tokenIds) payable {
        PositionState[] memory positions = getPositions(dex, npm, tokenIds);
        bytes memory returnData = abi.encode(positions);
        assembly ("memory-safe") {
            revert(add(returnData, 0x20), mload(returnData))
        }
    }

    /// @dev Public function to expose the abi for easier decoding using TypeChain
    /// @param dex DEX
    /// @param npm Nonfungible position manager
    /// @param tokenIds Token IDs of the positions
    function getPositions(
        DEX dex,
        address npm,
        uint256[] memory tokenIds
    ) public payable returns (PositionState[] memory positions) {
        unchecked {
            uint256 length = tokenIds.length;
            positions = new PositionState[](length);
            uint256 i;
            for (uint256 j; j < length; ++j) {
                uint256 tokenId = tokenIds[j];
                PositionState memory state = positions[i];
                if (positionInPlace(npm, tokenId, state.position)) {
                    ++i;
                    state.owner = NPMCaller.ownerOf(INPM(npm), tokenId);
                    peek(dex, npm, tokenId, state);
                }
            }
            assembly ("memory-safe") {
                mstore(positions, i)
            }
        }
    }
}
