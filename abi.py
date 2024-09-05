abi =  {
    "abi": [
      {
        "inputs": [
          {
            "internalType": "string",
            "name": "_orderId",
            "type": "string"
          }
        ],
        "name": "confirmReceipt",
        "outputs": [],
        "stateMutability": "nonpayable",
        "type": "function"
      },
      {
        "inputs": [
          {
            "internalType": "address",
            "name": "_seller",
            "type": "address"
          },
          {
            "internalType": "uint256",
            "name": "_buyerAmount",
            "type": "uint256"
          },
          {
            "internalType": "uint256",
            "name": "_sellerAmount",
            "type": "uint256"
          },
          {
            "internalType": "string",
            "name": "_orderId",
            "type": "string"
          }
        ],
        "name": "createEscrow",
        "outputs": [],
        "stateMutability": "nonpayable",
        "type": "function"
      },
      {
        "inputs": [
          {
            "internalType": "string",
            "name": "_orderId",
            "type": "string"
          }
        ],
        "name": "deposit",
        "outputs": [],
        "stateMutability": "payable",
        "type": "function"
      },
      {
        "inputs": [
          {
            "internalType": "string",
            "name": "_orderId",
            "type": "string"
          },
          {
            "internalType": "string",
            "name": "_disputeDetails",
            "type": "string"
          }
        ],
        "name": "dispute",
        "outputs": [],
        "stateMutability": "nonpayable",
        "type": "function"
      },
      {
        "inputs": [
          {
            "internalType": "string",
            "name": "_orderId",
            "type": "string"
          }
        ],
        "name": "refund",
        "outputs": [],
        "stateMutability": "nonpayable",
        "type": "function"
      },
      {
        "inputs": [
          {
            "internalType": "string",
            "name": "_orderId",
            "type": "string"
          },
          {
            "internalType": "address",
            "name": "faultParty",
            "type": "address"
          }
        ],
        "name": "resolveDispute",
        "outputs": [],
        "stateMutability": "nonpayable",
        "type": "function"
      },
      {
        "inputs": [],
        "stateMutability": "nonpayable",
        "type": "constructor"
      },
      {
        "inputs": [],
        "name": "contractOwner",
        "outputs": [
          {
            "internalType": "address",
            "name": "",
            "type": "address"
          }
        ],
        "stateMutability": "view",
        "type": "function"
      },
      {
        "inputs": [],
        "name": "DISPUTE_TIMEOUT",
        "outputs": [
          {
            "internalType": "uint256",
            "name": "",
            "type": "uint256"
          }
        ],
        "stateMutability": "view",
        "type": "function"
      },
      {
        "inputs": [
          {
            "internalType": "string",
            "name": "",
            "type": "string"
          }
        ],
        "name": "escrows",
        "outputs": [
          {
            "internalType": "address",
            "name": "buyer",
            "type": "address"
          },
          {
            "internalType": "address",
            "name": "seller",
            "type": "address"
          },
          {
            "internalType": "uint256",
            "name": "buyerAmount",
            "type": "uint256"
          },
          {
            "internalType": "uint256",
            "name": "sellerAmount",
            "type": "uint256"
          },
          {
            "internalType": "string",
            "name": "orderId",
            "type": "string"
          },
          {
            "internalType": "bool",
            "name": "isConfirmedByBuyer",
            "type": "bool"
          },
          {
            "internalType": "bool",
            "name": "isConfirmedBySeller",
            "type": "bool"
          },
          {
            "internalType": "bool",
            "name": "isDisputed",
            "type": "bool"
          },
          {
            "internalType": "uint256",
            "name": "disputeTimestamp",
            "type": "uint256"
          },
          {
            "internalType": "string",
            "name": "disputeDetails",
            "type": "string"
          },
          {
            "internalType": "enum Escrow.State",
            "name": "currentState",
            "type": "uint8"
          }
        ],
        "stateMutability": "view",
        "type": "function"
      },
      {
        "inputs": [],
        "name": "getAllOrderIds",
        "outputs": [
          {
            "internalType": "string[]",
            "name": "",
            "type": "string[]"
          }
        ],
        "stateMutability": "view",
        "type": "function"
      },
      {
        "inputs": [
          {
            "internalType": "string",
            "name": "_orderId",
            "type": "string"
          }
        ],
        "name": "getEscrowDetails",
        "outputs": [
          {
            "components": [
              {
                "internalType": "address",
                "name": "buyer",
                "type": "address"
              },
              {
                "internalType": "address",
                "name": "seller",
                "type": "address"
              },
              {
                "internalType": "uint256",
                "name": "buyerAmount",
                "type": "uint256"
              },
              {
                "internalType": "uint256",
                "name": "sellerAmount",
                "type": "uint256"
              },
              {
                "internalType": "string",
                "name": "orderId",
                "type": "string"
              },
              {
                "internalType": "bool",
                "name": "isConfirmedByBuyer",
                "type": "bool"
              },
              {
                "internalType": "bool",
                "name": "isConfirmedBySeller",
                "type": "bool"
              },
              {
                "internalType": "bool",
                "name": "isDisputed",
                "type": "bool"
              },
              {
                "internalType": "uint256",
                "name": "disputeTimestamp",
                "type": "uint256"
              },
              {
                "internalType": "string",
                "name": "disputeDetails",
                "type": "string"
              },
              {
                "internalType": "enum Escrow.State",
                "name": "currentState",
                "type": "uint8"
              }
            ],
            "internalType": "struct Escrow.EscrowDetails",
            "name": "",
            "type": "tuple"
          }
        ],
        "stateMutability": "view",
        "type": "function"
      },
      {
        "inputs": [
          {
            "internalType": "uint256",
            "name": "",
            "type": "uint256"
          }
        ],
        "name": "orderIds",
        "outputs": [
          {
            "internalType": "string",
            "name": "",
            "type": "string"
          }
        ],
        "stateMutability": "view",
        "type": "function"
      }
    ]
  }
