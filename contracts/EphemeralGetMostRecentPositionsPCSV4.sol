// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import {IERC721} from "@openzeppelin/contracts/token/ERC721/IERC721.sol";
import {ICLPositionManager} from "infinity-periphery/src/pool-cl/interfaces/ICLPositionManager.sol";

/// @notice A lens to return most recent positions by owner for PCS Infinity.
/// @author Aperture Finance
/// @dev The return data can be accessed externally by `eth_call` without a `to` address or internally by catching the
/// revert data, and decoded by `abi.decode(data, (uint256[]))`
contract EphemeralGetMostRecentPositionsPCSV4 {
    constructor(address npm, address owner, uint256 limit) payable {
        uint256[] memory positions = getMostRecentPositions(npm, owner, limit);
        bytes memory returnData = abi.encode(positions);
        assembly ("memory-safe") {
            // The return data in a constructor will be written to code, which may exceed the contract size limit.
            revert(add(returnData, 0x20), mload(returnData))
        }
    }

    /// @dev Public function to expose the abi for easier decoding using TypeChain
    /// @param npm Nonfungible position manager
    /// @param owner The address that owns the NFTs
    /// @param limit The maximum number of most recent positions to return
    function getMostRecentPositions(address npm, address owner, uint256 limit)
        public
        payable
        returns (uint256[] memory positions)
    {
        uint256 nextTokenId = ICLPositionManager(npm).nextTokenId();
        uint256[] memory recentPositions = new uint256[](limit);
        uint256 count = 0;
        unchecked {
            for (uint256 i = 0; i < limit; ++i) {
                uint256 tokenId = nextTokenId - 1 - i;
                if (tokenId == 0) {
                    break;
                }
                try IERC721(npm).ownerOf(tokenId) returns (address posOwner) {
                    if (posOwner != owner) {
                        continue;
                    }
                    recentPositions[count] = tokenId;
                    ++count;
                } catch {
                    // Token burned.
                    continue;
                }
            }
            // Resize the array to the actual count
            positions = new uint256[](count);
            for (uint256 j = 0; j < count; ++j) {
                positions[j] = recentPositions[j];
            }
        }
    }
}
