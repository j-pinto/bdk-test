use bdk_bitcoind_rpc::bitcoincore_rpc::{Auth, Client, RpcApi};
use bdk_wallet::bitcoin::{key::{self, Secp256k1}, NetworkKind};
use bdk_wallet::keys::bip39::Mnemonic;

fn connect_to_node() -> Client {
    let rpc_auth = Auth::UserPass(String::from("user"), String::from("password"));
    let rpc_url = String::from("http://192.168.1.2:48332");
    let rpc_client = Client::new(&rpc_url, rpc_auth).unwrap();
    rpc_client
}

fn main() {
    let client = connect_to_node();

    let priv_key = key::PrivateKey::generate(NetworkKind::Test);
    println!("private key: {}", priv_key);

    let secp = Secp256k1::new();

    let pub_key = priv_key.public_key(&secp);
    println!("public key: {}", pub_key);

    let entropy = priv_key.to_bytes();

    let mnemonic = Mnemonic::from_entropy(&entropy).unwrap();
    println!("mnemonic: {:?}", mnemonic);

	for (i, word) in mnemonic.words().enumerate() {
	    println!("{}. {}", i, word);
	}
}

