// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract Escrow {
    struct EscrowDetails {
        address buyer;
        address seller;
        address escrowAgent;
        uint256 amount;
        string orderId;
        bool isConfirmedByBuyer;
        bool isConfirmedBySeller;
        bool isDisputed;
        uint256 disputeTimestamp;
        string disputeDetails;
        State currentState;
    }

    enum State {
        Funded,
        Released,
        Refunded,
        Disputed
    }

    mapping(string => EscrowDetails) public escrows;
    uint256 public constant DISPUTE_TIMEOUT = 7 days;
    string[] public orderIds;

    modifier onlyBuyer(string memory _orderId) {
        require(
            msg.sender == escrows[_orderId].buyer,
            "Only the buyer can perform this action"
        );
        _;
    }

    modifier onlySeller(string memory _orderId) {
        require(
            msg.sender == escrows[_orderId].seller,
            "Only the seller can perform this action"
        );
        _;
    }

    modifier onlyEscrowAgent(string memory _orderId) {
        require(
            msg.sender == escrows[_orderId].escrowAgent,
            "Only the escrow agent can perform this action"
        );
        _;
    }

    modifier inState(string memory _orderId, State expectedState) {
        require(
            escrows[_orderId].currentState == expectedState,
            "Invalid state"
        );
        _;
    }

    function getAllOrderIds() external view returns (string[] memory) {
        return orderIds;
    }

    function createEscrow(
        address _seller,
        address _escrowAgent,
        string memory _orderId
    ) external {
        EscrowDetails memory newEscrow;
        newEscrow.buyer = msg.sender;
        newEscrow.seller = _seller;
        newEscrow.escrowAgent = _escrowAgent;
        newEscrow.orderId = _orderId;
        newEscrow.currentState = State.Funded;

        escrows[_orderId] = newEscrow;
        orderIds.push(_orderId);
    }

    function deposit(
        string memory _orderId
    ) external payable onlyBuyer(_orderId) inState(_orderId, State.Funded) {
        require(escrows[_orderId].amount == 0, "Already funded");
        escrows[_orderId].amount = msg.value;
    }

    function confirmReceipt(string memory _orderId) external {
        require(
            escrows[_orderId].currentState == State.Funded ||
                escrows[_orderId].currentState == State.Disputed,
            "Invalid state"
        );

        if (msg.sender == escrows[_orderId].buyer) {
            escrows[_orderId].isConfirmedByBuyer = true;
        } else if (msg.sender == escrows[_orderId].seller) {
            escrows[_orderId].isConfirmedBySeller = true;
        }

        if (
            escrows[_orderId].isConfirmedByBuyer &&
            escrows[_orderId].isConfirmedBySeller
        ) {
            releaseFunds(_orderId);
        }
    }

    function releaseFunds(
        string memory _orderId
    ) internal inState(_orderId, State.Funded) {
        require(
            escrows[_orderId].isConfirmedByBuyer &&
                escrows[_orderId].isConfirmedBySeller,
            "Both parties must confirm"
        );
        payable(escrows[_orderId].seller).transfer(escrows[_orderId].amount);
        escrows[_orderId].currentState = State.Released;
    }

    function refund(
        string memory _orderId
    ) external onlyEscrowAgent(_orderId) inState(_orderId, State.Funded) {
        payable(escrows[_orderId].buyer).transfer(escrows[_orderId].amount);
        escrows[_orderId].currentState = State.Refunded;
    }

    function dispute(
        string memory _orderId,
        string memory _disputeDetails
    ) external inState(_orderId, State.Funded) {
        require(
            msg.sender == escrows[_orderId].buyer ||
                msg.sender == escrows[_orderId].seller,
            "Not authorized"
        );
        escrows[_orderId].isDisputed = true;
        escrows[_orderId].disputeTimestamp = block.timestamp;
        escrows[_orderId].disputeDetails = _disputeDetails;
        escrows[_orderId].currentState = State.Disputed;
    }

    function resolveDispute(
        string memory _orderId,
        bool refundToBuyer
    ) external onlyEscrowAgent(_orderId) inState(_orderId, State.Disputed) {
        require(
            block.timestamp >=
                escrows[_orderId].disputeTimestamp + DISPUTE_TIMEOUT,
            "Dispute timeout not reached"
        );
        if (refundToBuyer) {
            payable(escrows[_orderId].buyer).transfer(escrows[_orderId].amount);
            escrows[_orderId].currentState = State.Refunded;
        } else {
            payable(escrows[_orderId].seller).transfer(
                escrows[_orderId].amount
            );
            escrows[_orderId].currentState = State.Released;
        }
        escrows[_orderId].isDisputed = false;
    }

    function getEscrowDetails(
        string memory _orderId
    ) external view returns (EscrowDetails memory) {
        return escrows[_orderId];
    }
}
