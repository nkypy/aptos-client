use aptos_sdk::types::account_address::AccountAddress;
use serde::{de::DeserializeOwned, Deserialize};

use crate::{move_types::U64, RestClient};

// #[derive(Debug, Deserialize)]
// pub struct TableItem<T> {
//     pub data: T,
// }

#[derive(Debug, Deserialize)]
pub struct CollectionData {
    pub name: String,
    pub description: String,
    pub uri: String,
    pub supply: U64,
}

#[derive(Debug, Deserialize)]
pub struct TokenData {
    pub name: String,
    pub description: String,
    pub uri: String,
    pub supply: U64,
    pub default_properties: TokenDataPropertyMap,
}

#[derive(Debug, Deserialize)]
pub struct TokenDataPropertyMap {
    pub map: TokenDataPropertyData,
}

#[derive(Debug, Deserialize)]
pub struct TokenDataPropertyData {
    pub data: Vec<TokenDataPropertyItem>,
}

#[derive(Debug, Deserialize)]
pub struct TokenDataPropertyItem {
    pub key: String,
    pub value: TokenDataPropertyItemValue,
}

#[derive(Debug, Deserialize)]
pub struct TokenDataPropertyItemValue {
    #[serde(rename = "type")]
    pub value_type: String,
    pub value: String,
}

#[derive(Debug, Deserialize)]
pub struct Token {
    pub amount: U64,
    pub id: TokenId,
}

#[derive(Debug, Deserialize)]
pub struct TokenId {
    pub property_version: U64,
    pub token_data_id: TokenDataId,
}

#[derive(Debug, Deserialize)]
pub struct TokenDataId {
    pub creator: AccountAddress,
    pub collection: String,
    pub name: String,
}

impl RestClient {
    // basic
    pub fn table_item(
        &self,
        table_handle: &str,
        key_type: &str,
        value_type: &str,
        key: serde_json::Value,
        _ledger_version: Option<U64>,
    ) -> Result<String, ureq::Error> {
        Ok(self
            .client
            .post(&format!("{}/tables/{}/item", self.base_url, table_handle))
            .send_json(serde_json::json!({
                "key_type": key_type,
                "value_type": value_type,
                "key": key,
            }))?
            .into_string()?)
    }

    pub fn table_item_json<T: DeserializeOwned>(
        &self,
        table_handle: &str,
        key_type: &str,
        value_type: &str,
        key: serde_json::Value,
        _ledger_version: Option<U64>,
    ) -> Result<T, ureq::Error> {
        Ok(self
            .client
            .post(&format!("{}/tables/{}/item", self.base_url, table_handle))
            .send_json(serde_json::json!({
                "key_type": key_type,
                "value_type": value_type,
                "key": key,
            }))?
            .into_json::<T>()?)
    }

    // public
    pub fn collection_data(
        &self,
        account_address: AccountAddress,
        collection_name: &str,
    ) -> Result<CollectionData, ureq::Error> {
        Ok(self.table_item_json::<CollectionData>(
            &self.account_resource_collection_data_handle(account_address)?,
            "0x1::string::String",
            "0x3::token::CollectionData",
            serde_json::json!(collection_name),
            None,
        )?)
    }

    pub fn token_data(
        &self,
        creator: AccountAddress,
        collection_name: &str,
        token_name: &str,
        _property_version: u64,
    ) -> Result<TokenData, ureq::Error> {
        let token_data_id = serde_json::json!({
            "creator": creator.to_hex_literal(),
            "collection": collection_name,
            "name": token_name,
        });
        Ok(self.table_item_json::<TokenData>(
            &self.account_resource_token_data_handle(creator)?,
            "0x3::token::TokenDataId",
            "0x3::token::TokenData",
            token_data_id,
            None,
        )?)
    }

    pub fn token(
        &self,
        owner: AccountAddress,
        creater: AccountAddress,
        collection_name: &str,
        token_name: &str,
        property_version: u64,
    ) -> Result<Token, ureq::Error> {
        let token_id = serde_json::json!({
            "token_data_id": {
                "creator": creater.to_hex_literal(),
                "collection": collection_name,
                "name": token_name,
            },
            "property_version": U64(property_version),
        });
        Ok(self.table_item_json::<Token>(
            &self.account_resource_tokens_handle(owner)?,
            "0x3::token::TokenId",
            "0x3::token::Token",
            token_id,
            None,
        )?)
    }
}
