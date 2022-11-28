use codec::Decode;
use sp_keyring::AccountKeyring;
use sp_runtime::{AccountId32 as AccountId, MultiAddress};
use std::{sync::mpsc::channel, thread};
use substrate_api_client::{rpc::WsRpcClient, Api, AssetTipExtrinsicParams, StaticEvent, XtStatus};

#[derive(Decode)]
struct TransferEventArgs {
    from: AccountId,
    to: AccountId,
    value: u128,
}

impl StaticEvent for TransferEventArgs {
    const PALLET: &'static str = "Balances";
    const EVENT: &'static str = "Transfer";
}

pub fn get_event(client: WsRpcClient) {
    let alice = AccountKeyring::Alice.pair();
    let api = Api::<_, _, AssetTipExtrinsicParams>::new(client)
        .map(|api| api.set_signer(alice.clone()))
        .unwrap();

    println!("Subscribe to events");

    let api2 = api.clone();
    // create new thread to catch the event 
    let thread_output = thread::spawn(move || {
        let (event_in, event_out) = channel();
        api2.subscribe_events(event_in).unwrap();
        let args: TransferEventArgs =
            api2.wait_for_event::<TransferEventArgs>(&event_out).unwrap();
        args
    });

    let bob = AccountKeyring::Bob.to_account_id();

    // Generate extrinsics
    let xt = api.balance_transfer(MultiAddress::Id(bob), 1000000000000);
    println!("[+] Composed extrinsic: {:?}\n", xt);

    let tx_hash = api.send_extrinsic(xt.hex_encode(), XtStatus::InBlock).unwrap().unwrap();
    println!("[+] Transaction got included. Hash: {:?}\n", tx_hash);

    let args = thread_output.join().unwrap();
    println!("Transactor: {:?}", args.from);
    println!("Destination: {:?}", args.to);
    println!("Value: {:?}", args.value);
}






