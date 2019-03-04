#![feature(proc_macro_hygiene, decl_macro)]


// #[macro_use] extern crate rocket;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
use std::process::Command;
//use serde::{Deserialize, Serialize};
use serde_json::Result;

//#[get("/")]
//fn index() -> &'static str {
//    "Hello, world!"
//}

fn main() {
    // rocket::ignite().mount("/", routes![index]).launch();

    lnd(String::from("listchannels"));
}


#[derive(Serialize, Deserialize)]
struct Channel {
    active: bool,
    remote_pubkey: String,
    channel_point: String,
    chan_id: String,
    capacity: String,
    local_balance: String,
    remote_balance: String,
    commit_fee: String,
    commit_weight: String,
    fee_per_kw: String,
    unsettled_balance: String,
    total_satoshis_sent: String,
    total_satoshis_received: String,
    num_updates: String,
    pending_htlcs: Vec<String>,
    csv_delay: u8,
    private: bool,
    initiator: bool
}

#[derive(Serialize, Deserialize)]
struct ListChannelsResult {
    channels: Vec<Channel>
}

fn lnd(command: String) -> Result<()> {
    let output = Command::new("lncli")
        .arg("--rpcserver=localhost:10001")
        .arg("--macaroonpath=~/projects/go/dev/alice/data/chain/bitcoin/testnet/admin.macaroon")
        .arg(command)
        .output().expect("{}");

    let body = String::from_utf8_lossy(&output.stdout);
    let parsed: ListChannelsResult = serde_json::from_str(&body)?;
    println!("remote_pubkey: {}", parsed.channels[0].remote_pubkey);

    Ok(())
}
