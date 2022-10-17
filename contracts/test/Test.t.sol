// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8;

import {ERC20} from "openzeppelin-contracts/token/ERC20/ERC20.sol";
import {console} from "forge-std/console.sol";
import {DSTest} from "forge-std/Test.sol";

contract MyERC20 is ERC20 {
    constructor() ERC20("Name", "SYM") {
        this;
    }
}

contract Test is DSTest {
    function test() public {
        ERC20 token = new MyERC20();
        assertEq(token.totalSupply(), 0);
    }
}
