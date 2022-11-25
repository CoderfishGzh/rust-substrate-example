#![feature(let_else)]
/*
    Copyright 2019 Supercomputing Systems AG
    Licensed under the Apache License, Version 2.0 (the "License");
    you may not use this file except in compliance with the License.
    You may obtain a copy of the License at

        http://www.apache.org/licenses/LICENSE-2.0

    Unless required by applicable law or agreed to in writing, software
    distributed under the License is distributed on an "AS IS" BASIS,
    WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
    See the License for the specific language governing permissions and
    limitations under the License.
*/

use sp_core::{sr25519, sr25519::Pair};
use sp_keyring::AccountKeyring;
use substrate_api_client::{
    compose_extrinsic, rpc::WsRpcClient, Api, AssetTipExtrinsicParams, GenericAddress,
    UncheckedExtrinsicV4, XtStatus,
};

fn main() {
    // 链接地址
    let Some(url) = get_url() else {
		println!("url is empty");
		return;
	};

    // initialize api and set the signer (sender) that is used to sign the extrinsics
    let from = AccountKeyring::Alice.pair();
    let client = WsRpcClient::new(&url);
    let api = Api::<_, _, AssetTipExtrinsicParams>::new(client)
        .map(|api| api.set_signer(from))
        .unwrap();

    // set the recipient
    let to = AccountKeyring::Bob.to_account_id();

    // call Balances::transfer
    // the names are given as strings
    #[allow(clippy::redundant_clone)]
        let xt: UncheckedExtrinsicV4<_, _> = compose_extrinsic!(
		api.clone(),
		"Balances",
		"transfer",
		GenericAddress::Id(to),
		Compact(42_u128)
	);

    println!("[+] Composed Extrinsic:\n {:?}\n", xt);

    // send and watch extrinsic until InBlock
    let tx_hash = api.send_extrinsic(xt.hex_encode(), XtStatus::Future);
    println!("[+] Transaction got included. Hash: {:?}", tx_hash);
}

// 返回链对外开放的地址
fn get_url() -> Option<String> {
    Some("ws://127.0.0.1:9944".to_string())
}
