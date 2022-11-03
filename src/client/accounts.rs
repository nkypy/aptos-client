use aptos_sdk::types::account_address::AccountAddress;
use serde::{de::DeserializeOwned, Deserialize};

use crate::{move_types::U64, RestClient};

/// GET /accounts/{address}
#[derive(Debug, Deserialize)]
pub struct Account {
    pub sequence_number: U64,
}

/// GET /accounts/{address}/resource/{resource_type}
#[derive(Debug, Deserialize)]
pub struct AccountResource<T> {
    #[serde(rename = "type")]
    pub resource_type: String,
    pub data: T,
}

//

#[derive(Debug, Deserialize)]
pub struct AptosCoin {
    pub value: U64,
}

#[derive(Debug, Deserialize)]
pub struct Balance {
    pub coin: AptosCoin,
}

//

#[derive(Debug, Deserialize)]
struct Handle {
    pub handle: String,
}

#[derive(Debug, Deserialize)]
struct CollectionData {
    pub collection_data: Handle,
}

#[derive(Debug, Deserialize)]
struct TokenData {
    pub token_data: Handle,
}

#[derive(Debug, Deserialize)]
struct Tokens {
    pub tokens: Handle,
}

//

impl RestClient {
    // basic
    pub fn account_resource<T: DeserializeOwned>(
        &self,
        account_address: AccountAddress,
        resource_type: &str,
    ) -> Result<AccountResource<T>, ureq::Error> {
        Ok(self
            .client
            .get(&format!(
                "{}/accounts/{}/resource/{}",
                self.base_url, account_address, resource_type
            ))
            .call()?
            .into_json::<AccountResource<T>>()?)
    }

    // private
    pub(crate) fn account_resource_collection_data_handle(
        &self,
        account_address: AccountAddress,
    ) -> Result<String, ureq::Error> {
        Ok(self
            .account_resource::<CollectionData>(account_address, "0x3::token::Collections")?
            .data
            .collection_data
            .handle)
    }

    pub(crate) fn account_resource_token_data_handle(
        &self,
        account_address: AccountAddress,
    ) -> Result<String, ureq::Error> {
        Ok(self
            .account_resource::<TokenData>(account_address, "0x3::token::Collections")?
            .data
            .token_data
            .handle)
    }

    pub(crate) fn account_resource_tokens_handle(
        &self,
        account_address: AccountAddress,
    ) -> Result<String, ureq::Error> {
        Ok(self
            .account_resource::<Tokens>(account_address, "0x3::token::TokenStore")?
            .data
            .tokens
            .handle)
    }

    // public
    pub fn account_sequence_number(
        &self,
        account_address: AccountAddress,
        _ledger_version: Option<U64>,
    ) -> Result<u64, ureq::Error> {
        Ok(self
            .client
            .get(&format!("{}/accounts/{}", self.base_url, account_address))
            .call()?
            .into_json::<Account>()?
            .sequence_number
            .0)
    }

    pub fn account_balance(&self, account_address: AccountAddress) -> Result<u64, ureq::Error> {
        Ok(self
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
