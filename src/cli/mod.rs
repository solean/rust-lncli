extern crate serde_json;
use serde_json::Result;
use std::process::Command;
use crate::types;


fn run_command(command: String) -> String {
    let output = Command::new("lncli")
        .arg("--rpcserver=localhost:10001")
        .arg("--macaroonpath=~/projects/go/dev/alice/data/chain/bitcoin/testnet/admin.macaroon")
        .arg(command)
        .output().expect("{}");

    return String::from_utf8_lossy(&output.stdout).to_string();
}

pub fn list_channels() -> Result<Vec<types::Channel>> {
    let body = run_command(String::from("listchannels"));
    let parsed: types::ListChannelsResult = serde_json::from_str(&body)?;
    Ok(parsed.channels)
}

pub fn get_wallet_balance() -> Result<types::WalletBalance> {
    let body = run_command(String::from("walletbalance"));
    let parsed: types::WalletBalance = serde_json::from_str(&body)?;
    Ok(parsed)
}

