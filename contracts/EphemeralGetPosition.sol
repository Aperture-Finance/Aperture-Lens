// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "./PositionUtils.sol";

/// @notice A lens for Uniswap v3 that peeks into the current state of position and pool info without deployment
/// @author Aperture Finance
/// @dev The return data can be accessed externally by `eth_call` without a `to` address or internally by catching the
/// revert data, and decoded by `abi.decode(data, (PositionState))`
contract EphemeralGetPosition is PositionUtils {
    constructor(INPM npm, uint256 tokenId) payable {
        PositionState memory pos = getPosition(npm, tokenId);
        bytes memory returnData = abi.encode(pos);
        assembly ("memory-safe") {
            revert(add(returnData, 0x20), mload(returnData))
        }
    }

    /// @dev Public function to expose the abi for easier decoding using TypeChain
    /// @param npm Nonfungible position manager
    /// @param tokenId Token ID of the position
    function getPosition(INPM npm, uint256 tokenId) public payable returns (PositionState memory state) {
        state.owner = NPMCaller.ownerOf(npm, tokenId);
        positionInPlace(npm, tokenId, state.position);
        peek(npm, tokenId, state);
    }
}
