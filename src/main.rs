#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate serde_derive;
extern crate serde_json;
use serde_json::Result;

mod cli;
mod types;


fn main() {
    let result: Result<Vec<types::Channel>> = cli::list_channels();
    let channels = result.unwrap();
    println!("{:#?}", channels[0]);
    println!("{:#?}", cli::get_wallet_balance().unwrap());
}

