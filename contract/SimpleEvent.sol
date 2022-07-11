pragma solidity ^0.5.0;

contract SimpleEvent {
    event Hello(address sender);

    constructor() {
        minter = msg.sender;
    }

    function hello() public {
        emit Hello(msg.sender);
    }
}
