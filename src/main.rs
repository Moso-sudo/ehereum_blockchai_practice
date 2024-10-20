use std::env;
use dotenv::dotenv;
use ethers::prelude::*;
use ethers::providers::{provider, Http};
use std::convert::TryFrom;
use std::sync::Arc;


#[tokio::main]
async fn main()->Result<(), Box<dyn std::error::Error>> {
    //load our environment variable 
   dotenv().ok();
   let ganache_url =env::var("GANACHE_URL").expect("GANACHE URL not found");
   let private_key_1 = env::var("PRIVATE_KEY_1").expect("WALLET 1 not found");
   let private_key_1 = env::var("PRIVATE_KEY_2").expect("WALLET 2 not found");

   if private_key_1.starts_with("0x"){
    private_key_1 =private_key_1.trim_end_matches("0x").to_string();
   }

   if private_key_2.starts_with("0x"){
    private_key_2 =private_key_2.trim_end_matches("0x").to_string();
   }

   //connect to ganache
   let provider =Provider::<Http>::try_from(ganache_url.as_str());
   let provider_arc =Arc:new(provider);

   //create wallets
   let wallet_1 = private_key_1.parse::<LocalWallet>()?.with_chain_id();
   let wallet_2 = private_key_2.parse::<LocalWallet>()?.with_chain_id();

   //get address
   let wallet_1_address =wallet_1.address();
   let wallet_1_balance= provider.get_balance(wallet_1_address, None).await?;

   println!("wallet  balance{}", wallet_1_balance)
   
   

}
