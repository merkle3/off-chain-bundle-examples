pragma solidity ^0.8.0;

import "forge-std/Test.sol";
import "../src/Basic.sol";

contract BasicTest is Test {
    Basic public basic;
 
    uint256 constant fees = 1;
    bytes constant data = hex"07";

    function setUp() public {
        basic = new Basic();
        vm.deal(address(basic), 1e18);
        vm.deal(address(0), 1e18);
    }

    function test_search() public {
        vm.prank(address(0));
        (bytes4 result_selector, uint256 result_fees, bytes memory result_data) = abi.decode(basic.search(fees, data), (bytes4, uint256, bytes));

        bytes4 settle_selector = basic.settle.selector;
        assertEq(result_selector, settle_selector);
        assertEq(result_fees, fees);
        assertEq(result_data, data);
    }

    function test_settle() public {
        vm.recordLogs();
        vm.prank(address(0));
        basic.settle{value: 1}(fees, data);
        Vm.Log[] memory logs = vm.getRecordedLogs();
        assertEq(logs[0].topics.length, 2);
        assertEq(logs[0].topics[0], keccak256("OnChainEvent(uint256,bytes)"));
        assertEq(logs[0].topics[1], bytes32(fees));
        assertEq(abi.decode(logs[0].data, (string)), string(data));
    }
}
