use serde::{Deserialize, Serialize};
use sha3::{Digest, Keccak256};
use std::time::{SystemTime, UNIX_EPOCH};

//Expo -- EXP Coin

struct UserAccount {
    public_key: Keccak256,
    token_balance: u64,
    epl_tokens: Vec<EplTokenInformation>,
}

struct EplTokenInformation {
    epl_token_name: String,
    epl_token_address: Keccak256,
    epl_token_balance: u64,
}

struct TransactionInformation {
    from_address: Keccak256,
    to_address: Keccak256,
    epl_token: EplTokenInformation,
    epl_token_transfer_amount: u64,
}
struct Block {
    block_hash: Keccak256,
    block_time_stamp: u128,
    transactions: Vec<TransactionInformation>,
    valid: bool,
}

fn main() {
    let from = Keccak256::new();
    from.finalize();

    let to = Keccak256::new();
    to.finalize();

    let token = Keccak256::new();
    token.finalize();
}
