use alloy::{
    contract::{ContractInstance, Interface},
    dyn_abi::DynSolValue,
    network::{Ethereum, EthereumWallet,
    TransactionBuilder},
    primitives::{address, hex, Address, U256},
    providers::{Provider,ProviderBuilder, RootProvider},
    signers::local::PrivateKeySigner,
    transports::http::{Client, Http},
    
    
};

use serde_json;
use std::{fs::File, str::FromStr};
use std::io::Read;

pub async fn load_contract() -> Result<ContractInstance<Http<Client>, RootProvider<Http<Client>>, Ethereum>, eyre::Error> {
    let mut file = File::open("Escrow.json")?;
    let mut abi_value = String::new();
    file.read_to_string(&mut abi_value)?;
    let abi = serde_json::from_str(&abi_value.to_string())?;

    let rpc_url = "https://a.sentry.testnet.kiivalidator.com:8645/".parse()?;
    let provider = ProviderBuilder::new().on_http(rpc_url);
    let contract_address = address!("672a8BD9BC91009Ff2feC43Cc173Bda3683C3378");
    
    let contract: ContractInstance<Http<Client>, RootProvider<Http<Client>>, Ethereum> =
        ContractInstance::new(contract_address, provider.clone(), Interface::new(abi));

    Ok(contract)
}

async fn create_escrow(contract_instance: &ContractInstance<Http<Client>, RootProvider<Http<Client>>, Ethereum>,
    buyer_address: Address,
    seller: Address,
    buyer_amount: u128, 
    seller_amount: u128,
    order_id: String,
    wallet: EthereumWallet,
) -> Result<()> {
    let _buyer_address = DynSolValue::from(buyer_address);
    let _seller_address = DynSolValue::from(seller);
    let _buyer_amount = DynSolValue::from(U256::from(buyer_amount));
    let _seller_amount = DynSolValue::from(U256::from(seller_amount));
    let _order_id = DynSolValue::from(order_id);

    let tx = contract_instance
        .function("createEscrow", &[
            _buyer_address,
            _seller_address,
            _buyer_amount,
            _seller_amount,
            _order_id,
        ])?.into_transaction_request().build(&wallet)
        .await;


    println!("tx_hash: {:?}", tx.unwrap().tx_hash());
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let contract = load_contract().await?;
    let buyer_address = address!("104dc4c1FcA6359B9bdBf81705E34f1ba91a3958");
    let seller = address!("F890F95047D40e59c42a3E6d5720a89EE29453cE");
    let seller_private_key = PrivateKeySigner::from_str("3f7e255c7a960413344eb493980a17696d97f94285b443891184aa15d767a04d").unwrap();
    let wallet = EthereumWallet::from(seller_private_key);
    let buyer_amount = 220;
    let seller_amount = 200;
    let order_id = "test".to_string();
    
    create_escrow(&contract, buyer_address, seller, buyer_amount, seller_amount, order_id,wallet).await?;
    Ok(())
}




