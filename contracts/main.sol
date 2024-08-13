// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract Escrow {
    struct EscrowDetails {
        address buyer;
        address seller;
        uint256 buyerAmount;
        uint256 sellerAmount;
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
    address public contractOwner;
    string[] public orderIds;

    constructor() {
        contractOwner = msg.sender;
    }

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

    modifier onlyContractOwner() {
        require(
            msg.sender == contractOwner,
            "Only the contract owner can perform this action"
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
        uint256 _buyerAmount,
        uint256 _sellerAmount,
        string memory _orderId
    ) external {
        EscrowDetails memory newEscrow;
        newEscrow.buyer = msg.sender;
        newEscrow.seller = _seller;
        newEscrow.buyerAmount = _buyerAmount;
        newEscrow.sellerAmount = _sellerAmount;
        newEscrow.orderId = _orderId;
        newEscrow.currentState = State.Funded;

        escrows[_orderId] = newEscrow;
        orderIds.push(_orderId);
    }

    function deposit(
        string memory _orderId
    ) external payable inState(_orderId, State.Funded) {
        if (msg.sender == escrows[_orderId].buyer) {
            require(msg.value == escrows[_orderId].buyerAmount, "Incorrect amount");
            escrows[_orderId].buyerAmount = msg.value;
        } else if (msg.sender == escrows[_orderId].seller) {
            require(msg.value == escrows[_orderId].sellerAmount, "Incorrect amount");
            escrows[_orderId].sellerAmount = msg.value;
        } else {
            revert("Only buyer or seller can deposit");
        }
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
        uint256 totalAmount = escrows[_orderId].buyerAmount + escrows[_orderId].sellerAmount;
        payable(escrows[_orderId].seller).transfer(totalAmount);
        escrows[_orderId].currentState = State.Released;
    }

    function refund(
        string memory _orderId
    ) external onlyContractOwner inState(_orderId, State.Funded) {
        payable(escrows[_orderId].buyer).transfer(escrows[_orderId].buyerAmount);
        payable(escrows[_orderId].seller).transfer(escrows[_orderId].sellerAmount);
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
        address faultParty
    ) external onlyContractOwner inState(_orderId, State.Disputed) {
        require(
            block.timestamp >= escrows[_orderId].disputeTimestamp + DISPUTE_TIMEOUT,
            "Dispute timeout not reached"
        );

        uint256 faultPartyAmount = faultParty == escrows[_orderId].buyer ? escrows[_orderId].buyerAmount : escrows[_orderId].sellerAmount;
        uint256 otherPartyAmount = faultParty == escrows[_orderId].buyer ? escrows[_orderId].sellerAmount : escrows[_orderId].buyerAmount;

        uint256 faultPartyRefund = (faultPartyAmount * 90) / 100;
        uint256 otherPartyReward = (faultPartyAmount * 9) / 100;
        uint256 fee = faultPartyAmount / 100;

        payable(contractOwner).transfer(fee);

        if (faultParty == escrows[_orderId].buyer) {
            payable(escrows[_orderId].buyer).transfer(faultPartyRefund);
            payable(escrows[_orderId].seller).transfer(otherPartyAmount + otherPartyReward);
        } else {
            payable(escrows[_orderId].seller).transfer(faultPartyRefund);
            payable(escrows[_orderId].buyer).transfer(otherPartyAmount + otherPartyReward);
        }

        escrows[_orderId].currentState = State.Released;
        escrows[_orderId].isDisputed = false;
    }

    function getEscrowDetails(
        string memory _orderId
    ) external view returns (EscrowDetails memory) {
        return escrows[_orderId];
    }
}
