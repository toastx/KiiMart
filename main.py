from abi import abi
import web3

ca = "0x672a8BD9BC91009Ff2feC43Cc173Bda3683C3378"
abi = abi["abi"]

rpc = "https://a.sentry.testnet.kiivalidator.com:8645/"
chain_id = 123454321

web3 = web3.Web3(web3.HTTPProvider(rpc)) 

contract = web3.eth.contract(address = ca, abi = abi)

seller_private_key = "3f7e255c7a960413344eb493980a17696d97f94285b443891184aa15d767a04d"

def create_escrow(buyer_address, seller_address, buyer_amount, seller_amount, order_id):
    txn = contract.functions.createEscrow(seller_address, buyer_amount, seller_amount, order_id).build_transaction({
        'from': seller_address,
        'chainId': chain_id,  
        'gas': 2000000,
        'gasPrice': web3.to_wei('20', 'gwei'),
        'nonce': web3.eth.get_transaction_count(buyer_address),
    })
    return txn


def deposit(address, order_id, amount):
    txn = contract.functions.deposit(order_id).build_transaction({
        'from': address,
        'chainId': chain_id,
        'gas': 2000000,
        'gasPrice': web3.toWei('20', 'gwei'),
        'nonce': web3.eth.getTransactionCount(address),
        'value': web3.toWei(amount, 'ether'),
    })
    return txn


def get_escrow_details(order_id):
    escrow_details = contract.functions.getEscrowDetails(order_id).call()
    return escrow_details


def confirm_receipt(address, order_id):
    txn = contract.functions.confirmReceipt(order_id).buildTransaction({
        'from': address,
        'chainId': chain_id,
        'gas': 2000000,
        'gasPrice': web3.toWei('20', 'gwei'),
        'nonce': web3.eth.getTransactionCount(address),
    })
    return txn


x = create_escrow("0x104dc4c1FcA6359B9bdBf81705E34f1ba91a3958","0xF890F95047D40e59c42a3E6d5720a89EE29453cE",330,300,"fg981")

signed_txn = web3.eth.account.sign_transaction(unsigned_txn, private_key=seller_private_key)

txn_hash = web3.eth.send_raw_transaction(signed_txn.rawTransaction)

txn_receipt = web3.eth.wait_for_transaction_receipt(txn_hash)

print(f"Transaction successful with hash: {txn_hash.hex()}")