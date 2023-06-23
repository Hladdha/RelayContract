// SPDX-License-Identifier: MIT
pragma solidity 0.8.19;

import "@openzeppelin/contracts/token/ERC20/ERC20.sol";

contract EfiToken is ERC20 {

    address public owner;

    modifier OnlyOwner {
        require(msg.sender == owner, "not authorized");
        _;
    }

    constructor () ERC20("Enfi", "efi") {
        owner = msg.sender;
    }
    
    function mint(address _to, uint256 _amount) external OnlyOwner {
       _mint(_to, _amount);
    }
}