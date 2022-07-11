use std::str::FromStr;
use ethereum_types::H160;
use hex_literal::hex;

use secp256k1::SecretKey;
use web3::{
    contract::{Contract, Options},
    types::U256,
};

#[tokio::main]
async fn main() -> web3::contract::Result<()> {
    let _ = env_logger::try_init();
    let http = web3::transports::Http::new("http://localhost:7545")?;
    let web3 = web3::Web3::new(http);
    let my_account:H160 = hex!("019848CF1879e3F1e8f462958aD1f62b15ec93F0").into();
    let my_priv = SecretKey::from_str("1052c7066e00eae264c9d1088ee4151412ea2ecd35a10d924d1921b024cc0508").unwrap();
    println!("1");
    // Get the contract bytecode for instance from Solidity compiler
    let bytecode = include_str!("../contract/contract.bytecode").trim_end();
    // Deploying a contract
    let contract = Contract::deploy(web3.eth(), include_bytes!("../contract/ABI.json"))?
        .confirmations(0)
        .options(Options::with(|opt| {
            opt.value = Some(5u32.into());
            opt.gas_price = Some(5u32.into());
            opt.gas = Some(3_000_000u32.into());
        }))
        .sign_with_key_and_execute(
            bytecode,
            (ethabi::Token::Address(my_account)),
            &my_priv,
            5777u64.into(),
        )
        .await?;
    println!("1");
    
    let result:web3::contract::Result<U256> = 
    contract.query("balanceOf", (my_account,), None, Options::default(), None).await;
    // Make sure to specify the expected return type, to prevent ambiguous compiler
    // errors about `Detokenize` missing for `()`.
    println!("{:?}", result);
    // assert_eq!(balance_of, 1_000_000.into());
    
    // // Accessing existing contract
    let contract_address = contract.address();
    println!("{}", contract_address);
    // let contract = Contract::from_json(
    //     web3.eth(),
    //     contract_address,
    //     include_bytes!("../src/contract/res/token.json"),
    // )?;

    // let result = contract.query("balanceOf", (my_account,), None, Options::default(), None);
    // let balance_of: U256 = result.await?;
    // assert_eq!(balance_of, 1_000_000i64.into());

    Ok(())
}
