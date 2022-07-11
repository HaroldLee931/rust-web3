use std::str::FromStr;

use secp256k1::SecretKey;

use web3::{
    ethabi::ethereum_types::U256,
    types::{Address, TransactionParameters},
};

/// Below generates and signs a transaction offline, before transmitting it to a public node (eg Infura)
/// For sending a transaction to a local node that stores private keys (eg Ganache) see transaction_private
#[tokio::main]
async fn main() -> web3::Result {
    // Sign up at infura > choose the desired network (eg Rinkeby) > copy the endpoint url into the below
    // If you need test ether use a faucet, eg https://faucet.rinkeby.io/
    // let transport = web3::transports::Http::new("https://rinkeby.infura.io/v3/XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX")?;
    // let web3 = web3::Web3::new(transport);

    let http = web3::transports::Http::new("http://localhost:7545")?;
    let web3 = web3::Web3::new(http);

    // Insert the 20-byte "to" address in hex format (prefix with 0x)
    let to = Address::from_str("0x7DA5670424475a74E04ebb88e7FC4256C3D5E610").unwrap();

    // Insert the 32-byte private key in hex format (do NOT prefix with 0x)
    let prvk = SecretKey::from_str("1052c7066e00eae264c9d1088ee4151412ea2ecd35a10d924d1921b024cc0508").unwrap();

    // Build the tx object
    let tx_object = TransactionParameters {
        to: Some(to),
        value: U256::exp10(18), //0.1 eth
        ..Default::default()
    };

    // Sign the tx (can be done offline)
    let signed = web3.accounts().sign_transaction(tx_object, &prvk).await?;

    // Send the tx to infura
    let result = web3.eth().send_raw_transaction(signed.raw_transaction).await?;

    println!("Tx succeeded with hash: {}", result);

    Ok(())
}
