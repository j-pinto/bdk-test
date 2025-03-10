use bdk_bitcoind_rpc::bitcoincore_rpc::{Auth, Client, RpcApi};

fn connect_to_node() -> Client {
    let rpc_auth = Auth::UserPass(String::from("user"), String::from("password"));
    let rpc_url = String::from("http://192.168.1.2:48332");
    let rpc_client = Client::new(&rpc_url, rpc_auth).unwrap();
    rpc_client
}

fn main() {
    let client = connect_to_node();
    println!("{:?}", client.get_block_count());
}

