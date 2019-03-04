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

    list_channels();
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
    csv_delay: u32,
    private: bool,
    initiator: bool
}

#[derive(Serialize, Deserialize)]
struct ListChannelsResult {
    channels: Vec<Channel>
}

#[derive(Serialize, Deserialize)]
struct WalletBalance {
    total_balance: i64,
    confirmed_balance: i64,
    unconfirmed_balance: i64
};

#[derive(Serialize, Deserialize)]
struct ChannelBalance {
    balance: i64,
    pending_open_balance: i64
};

#[derive(Serialize, Deserialize)]
struct Transaction {
    tx_hash: String,
    amount: i64,
    num_confirmations: i32,
    block_hash: String,
    block_height: i32,
    time_stamp: i64,
    total_fees: i64,
    dest_addresses: Vec<String>
}

#[derive(Serialize, Deserialize)]
struct Transactions {
    transactions: Vec<Transaction>
}

struct Outpoint {
    txid_bytes: u64,
    txid_str: String,
    output_index: u32
}

#[derive(Serialize, Deserialize)]
struct Utxo {
    type: String,
    address: String,
    amount_sat: i64,
    script_pubkey: String,
    outpoint: Outpoint,
    confirmations: i64
}

struct Peer {
    pub_key: String,
    address: String,
    bytes_sent: u64,
    bytes_recv: u64,
    sat_sent: i64,
    sat_recv: i64,
    inbound: bool,
    ping_time: i64
}



fn lncli(command: String) -> String {
    let output = Command::new("lncli")
        .arg("--rpcserver=localhost:10001")
        .arg("--macaroonpath=~/projects/go/dev/alice/data/chain/bitcoin/testnet/admin.macaroon")
        .arg(command)
        .output().expect("{}");

    let body = String::from_utf8_lossy(&output.stdout);
    // let parsed: ListChannelsResult = serde_json::from_str(&body)?;
    // println!("remote_pubkey: {}", parsed.channels[0].remote_pubkey);

    //Ok(())
    return body;
}

fn list_channels() -> Result<Vec<Channel>> {
    lncli(String::from("listchannels"));
    let parsed: ListChannelsResult = serde_json::from_str(&body)?;
    Ok(parsed.channels)
}

