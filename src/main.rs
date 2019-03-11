#![feature(proc_macro_hygiene, decl_macro)]


// #[macro_use] extern crate rocket;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
use std::process::Command;
//use serde::{Deserialize, Serialize};
use serde_json::Result;

mod types;

//#[get("/")]
//fn index() -> &'static str {
//    "Hello, world!"
//}

fn main() {
    // rocket::ignite().mount("/", routes![index]).launch();

    let result: Result<Vec<types::Channel>> = list_channels();
    let channels = result.unwrap();
    println!("{:#?}", channels[0]);
    println!("{:#?}", get_wallet_balance().unwrap());
}

fn lncli(command: String) -> String {
    let output = Command::new("lncli")
        .arg("--rpcserver=localhost:10001")
        .arg("--macaroonpath=~/projects/go/dev/alice/data/chain/bitcoin/testnet/admin.macaroon")
        .arg(command)
        .output().expect("{}");

    return String::from_utf8_lossy(&output.stdout).to_string();
}

fn list_channels() -> Result<Vec<types::Channel>> {
    let body = lncli(String::from("listchannels"));
    let parsed: types::ListChannelsResult = serde_json::from_str(&body)?;
    Ok(parsed.channels)
}

fn get_wallet_balance() -> Result<types::WalletBalance> {
    let body = lncli(String::from("walletbalance"));
    let parsed: types::WalletBalance = serde_json::from_str(&body)?;
    Ok(parsed)
}

