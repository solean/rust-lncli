extern crate serde_derive;


#[derive(Serialize, Deserialize, Debug)]
pub struct Channel {
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

#[derive(Serialize, Deserialize, Debug)]
pub struct ListChannelsResult {
    pub channels: Vec<Channel>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WalletBalance {
    total_balance: String, // i64
    confirmed_balance: String, // i64
    unconfirmed_balance: String // i64
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChannelBalance {
    balance: String, // i64
    pending_open_balance: String // i64
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Transaction {
    tx_hash: String,
    amount: i64,
    num_confirmations: i32,
    block_hash: String,
    block_height: i32,
    time_stamp: i64,
    total_fees: i64,
    dest_addresses: Vec<String>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Transactions {
    transactions: Vec<Transaction>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Outpoint {
    txid_bytes: u64,
    txid_str: String,
    output_index: u32
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Peer {
    pub_key: String,
    address: String,
    bytes_sent: u64,
    bytes_recv: u64,
    sat_sent: i64,
    sat_recv: i64,
    inbound: bool,
    ping_time: i64
}

