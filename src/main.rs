#![feature(let_else)]

use sp_core::sr25519;
use sp_keyring::AccountKeyring;
use substrate_api_client::{rpc::WsRpcClient, Api, AssetTipExtrinsicParams};

fn main() {
    let Some(url) = init::get_url() else {
		println!("url is empty");
		return;
	};

    // initialize api and set the signer (sender) that is used to sign the extrinsics
    let sender = AccountKeyring::Alice.pair();
    let client = WsRpcClient::new(&url);

    // send extrinsics
    extrinsic::send_extrinsic(client.clone(), sender);

    // check the storage
    let api = Api::<sr25519::Pair, _, AssetTipExtrinsicParams>::new(client.clone()).unwrap();
    let something: u32 = api
        .get_storage_value("Provider", "Something", None)
        .unwrap()
        .unwrap();

    assert_eq!(something, 10);

    // test event 
    event::get_event(client);
}
