use aptos_sdk::types::account_address::AccountAddress;
use serde::Deserialize;

use crate::{client::Client, types::U64};

#[derive(Debug)]
pub struct CoinClient {
    pub client: Client,
}

impl CoinClient {
    pub fn new(rest_url: &str) -> Self {
        Self {
            client: Client::new(rest_url),
        }
    }

    // APT 余额
    pub fn account_balance(&self, account_address: AccountAddress) -> Result<u64, ureq::Error> {
        Ok(self
            .client
            .account_resource::<Balance>(
                account_address,
                "0x1::coin::CoinStore<0x1::aptos_coin::AptosCoin>",
            )?
            .data
            .coin
            .value
            .0)
    }
}

#[derive(Debug, Deserialize)]
pub struct AptosCoin {
    pub value: U64,
}

#[derive(Debug, Deserialize)]
pub struct Balance {
    pub coin: AptosCoin,
}
