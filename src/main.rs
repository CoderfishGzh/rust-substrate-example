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
use substrate_api_client::{rpc::WsRpcClient, Api, AssetTipExtrinsicParams, Metadata};

fn main() {
    // 链接地址
    let Some(url) = get_url() else {
		println!("url is empty");
		return;
	};
    // 创建API
    let Ok(api) = get_api(url) else {
		println!("can not get api");
		return;
	};
    // 获取元数据
    let meta = Metadata::try_from(api.get_metadata().unwrap()).unwrap();
    meta.print_overview();
    meta.print_pallets();
    meta.print_pallets_with_calls();
    meta.print_pallets_with_events();
    meta.print_pallets_with_errors();
    meta.print_pallets_with_constants();

    // Print full substrate metadata json formatted.
    println!(
        "{}",
        Metadata::pretty_format(&api.get_metadata().unwrap()).unwrap()
    )
}

// 返回链对外开放的地址
fn get_url() -> Option<String> {
    Some("ws://127.0.0.1:9944".to_string())
}

// 创建client 以及 api ，返回api对象
fn get_api(url: String) -> Result<Api<Pair, WsRpcClient, AssetTipExtrinsicParams>, String> {
    let client = WsRpcClient::new(&url);
    let Ok(api) = Api::<sr25519::Pair, _, AssetTipExtrinsicParams>::new(client) else {
		return Err("create api error".to_string());
	};
    Ok(api)
}
