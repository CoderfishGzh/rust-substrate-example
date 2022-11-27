use sp_core::{sr25519, sr25519::Pair};
use substrate_api_client::{
    compose_extrinsic, rpc::WsRpcClient, Api, AssetTipExtrinsicParams, GenericAddress,
    UncheckedExtrinsicV4, XtStatus, compose_call, Metadata
};

pub fn send_extrinsic(client: WsRpcClient, sender: Pair) {
    // create api
    let api = Api::<_, _, AssetTipExtrinsicParams>::new(client)
        .map(|api| api.set_signer(sender))
        .unwrap();

     // use api call the extrinsics
    #[allow(clippy::redundant_clone)]
        let xt: UncheckedExtrinsicV4<_, _> = compose_extrinsic!(
		api.clone(),
		"Provider",
		"test"
	);
    println!("[+] Composed Extrinsic:\n {:?}\n", xt);

    // send and watch extrinsic until InBlock
    let tx_hash = api
        .send_extrinsic(xt.hex_encode(), XtStatus::InBlock)
        .unwrap();
    println!("[+] Transaction got included. Hash: {:?}", tx_hash);
}