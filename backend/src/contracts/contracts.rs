use alloy::{
    contract::{ContractInstance, Interface},
    dyn_abi::{parser::Error, DynSolValue},
    json_abi::JsonAbi,
    network::{Ethereum, EthereumWallet, NetworkWallet, TransactionBuilder},
    primitives::{address, hex, Address, U256},
    providers::{Provider,ProviderBuilder, RootProvider},
    signers::{k256::{ecdsa::SigningKey, Secp256k1}, local::{LocalSigner, PrivateKeySigner}},
    transports::http::{Client, Http},

};
use eyre::*;
use serde_json::{self, Value};
use std::{fs::File, str::FromStr};
use std::io::Read;

pub async fn load_contract() -> Result<ContractInstance<Http<Client>, RootProvider<Http<Client>>, Ethereum>, eyre::Error> {

    let mut file = File::open("D:/hackahton/rwamarketplace/backend/src/contracts/Escrow.json")?;
    println!("File opened");
    let mut abi_value = String::new();
    file.read_to_string(&mut abi_value)?;
    let json: serde_json::Value = serde_json::from_str(&abi_value)?;
    let abi: JsonAbi  = serde_json::from_value(json["abi"].clone())?;

    let rpc_url = "https://a.sentry.testnet.kiivalidator.com:8645/".parse()?;
    let provider = ProviderBuilder::new().on_http(rpc_url);
    let contract_address = address!("672a8BD9BC91009Ff2feC43Cc173Bda3683C3378");
    
    let contract: ContractInstance<Http<Client>, RootProvider<Http<Client>>, Ethereum> =
        ContractInstance::new(contract_address, provider.clone(), Interface::new(abi));

    Ok(contract)
}

pub async fn create_escrow(contract_instance: &ContractInstance<Http<Client>, RootProvider<Http<Client>>, Ethereum>,
    buyer_address: Address,
    seller: Address,
    buyer_amount: u128, 
    seller_amount: u128,
    order_id: String,
    wallet: EthereumWallet,
) -> Result<(),Report> {
    let _seller_address = DynSolValue::from(seller);
    let _buyer_amount = DynSolValue::from(U256::from(buyer_amount));
    let _seller_amount = DynSolValue::from(U256::from(seller_amount));
    let _order_id = DynSolValue::from(order_id);
    let provider = contract_instance.provider();
    let nonce = provider.get_transaction_count(seller).await.map_err(|e| eyre::ErrReport::msg(e.to_string()));
    let tx = contract_instance
        .function("createEscrow", &[
            _seller_address,
            _buyer_amount,
            _seller_amount,
            _order_id,
        ]).unwrap().into_transaction_request()
        .from(seller)
        .with_chain_id(123454321)
        .nonce(nonce.unwrap())
        .gas_limit(2000000)
        .max_fee_per_gas(100_000_000_000u64.into())
        .max_priority_fee_per_gas(2_000_000_000u64.into())
        .build(&wallet)
        .await;

    let signed_tx = provider.send_tx_envelope(tx.unwrap()).await;
    println!("tx_hash: {:?}", signed_tx.unwrap().tx_hash());
    Ok(())
}

pub async fn resolve_dispute(
    contract_instance: &ContractInstance<Http<Client>, RootProvider<Http<Client>>, Ethereum>,
    order_id: String,
    fault_party: Address,
    wallet: EthereumWallet,
) -> Result<(), Report> {
    let _order_id = DynSolValue::from(order_id);
    let _fault_party = DynSolValue::from(fault_party);
    let provider = contract_instance.provider();
    let tx = contract_instance
        .function("resolveDispute", &[_order_id, _fault_party])
        .unwrap()
        .into_transaction_request()
        .from(wallet.address())
        .with_chain_id(123454321)
        .gas_limit(2000000)
        .max_fee_per_gas(100_000_000_000u64.into())
        .max_priority_fee_per_gas(2_000_000_000u64.into())
        .build(&wallet)
        .await;

    let signed_tx = provider.send_tx_envelope(tx.unwrap()).await;
    println!("tx_hash: {:?}", signed_tx.unwrap().tx_hash());
    Ok(())
}

pub async fn refund(
    contract_instance: &ContractInstance<Http<Client>, RootProvider<Http<Client>>, Ethereum>,
    order_id: String,
    wallet: EthereumWallet,
) -> Result<(), Report> {
    let _order_id = DynSolValue::from(order_id);
    let provider = contract_instance.provider();
    let tx = contract_instance
        .function("refund", &[_order_id])
        .unwrap()
        .into_transaction_request()
        .from(wallet.address())
        .with_chain_id(123454321)
        .gas_limit(2000000)
        .max_fee_per_gas(100_000_000_000u64.into())
        .max_priority_fee_per_gas(2_000_000_000u64.into())
        .build(&wallet)
        .await;

    let signed_tx = provider.send_tx_envelope(tx.unwrap()).await;
    println!("tx_hash: {:?}", signed_tx.unwrap().tx_hash());
    Ok(())
}

pub async fn dispute(
    contract_instance: &ContractInstance<Http<Client>, RootProvider<Http<Client>>, Ethereum>,
    order_id: String,
    dispute_details: String,
    wallet: EthereumWallet,
) -> Result<(), Report> {
    let _order_id = DynSolValue::from(order_id);
    let _dispute_details = DynSolValue::from(dispute_details);
    let provider = contract_instance.provider();
    let tx = contract_instance
        .function("dispute", &[_order_id, _dispute_details])
        .unwrap()
        .into_transaction_request()
        .from(wallet.address())
        .with_chain_id(123454321)
        .gas_limit(2000000)
        .max_fee_per_gas(100_000_000_000u64.into())
        .max_priority_fee_per_gas(2_000_000_000u64.into())
        .build(&wallet)
        .await;

    let signed_tx = provider.send_tx_envelope(tx.unwrap()).await;
    println!("tx_hash: {:?}", signed_tx.unwrap().tx_hash());
    Ok(())
}

pub async fn deposit(
    contract_instance: &ContractInstance<Http<Client>, RootProvider<Http<Client>>, Ethereum>,
    order_id: String,
    value: u128, // Ether value to be deposited
    wallet: EthereumWallet,
) -> Result<(), Report> {
    let _order_id = DynSolValue::from(order_id);
    let provider = contract_instance.provider();
    let tx = contract_instance
        .function("deposit", &[_order_id])
        .unwrap()
        .into_transaction_request()
        .from(wallet.address())
        .value(U256::from(value)) // Adding the value to the transaction
        .with_chain_id(123454321)
        .gas_limit(2000000)
        .max_fee_per_gas(100_000_000_000u64.into())
        .max_priority_fee_per_gas(2_000_000_000u64.into())
        .build(&wallet)
        .await;

    let signed_tx = provider.send_tx_envelope(tx.unwrap()).await;
    println!("tx_hash: {:?}", signed_tx.unwrap().tx_hash());
    Ok(())
}


pub fn generate_private_key(key:&str)-> LocalSigner<SigningKey> {
    let key = PrivateKeySigner::from_str(key).unwrap();
    key
}

pub fn generate_wallet_obj(key:LocalSigner<SigningKey>) -> EthereumWallet {
    let wallet = EthereumWallet::from(key);
    wallet
}






