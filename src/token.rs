use crate::{client::Client, types::U64};
use aptos_sdk::{
    bcs,
    move_types::{identifier::Identifier, language_storage::ModuleId},
    types::{
        account_address::AccountAddress,
        transaction::{EntryFunction, TransactionPayload},
        LocalAccount,
    },
};
use serde::Deserialize;

#[derive(Debug)]
pub struct TokenClient {
    pub client: Client,
}

impl TokenClient {
    pub fn new(rest_url: &str) -> Self {
        Self {
            client: Client::new(rest_url),
        }
    }

    // 创建 collection
    pub fn create_collection(
        &self,
        account: LocalAccount,
        name: &str,
        description: &str,
        uri: &str,
    ) -> Result<String, ureq::Error> {
        let payload = TransactionPayload::EntryFunction(EntryFunction::new(
            ModuleId::new(
                AccountAddress::from_hex_literal("0x3").unwrap(),
                Identifier::new("token").unwrap(),
            ),
            Identifier::new("create_collection_script").unwrap(),
            vec![],
            vec![
                bcs::to_bytes(name).unwrap(),
                bcs::to_bytes(description).unwrap(),
                bcs::to_bytes(uri).unwrap(),
                bcs::to_bytes(&u64::MAX).unwrap(),
                bcs::to_bytes(&vec![false, false, false]).unwrap(),
            ],
        ));
        let signed_transaction = self
            .client
            .create_single_signer_bcs_transaction(account, payload);
        Ok(self.client.submit_bcs_transaction(signed_transaction)?)
    }

    // 创建 token
    pub fn create_token(
        &self,
        account: LocalAccount,
        collection_name: &str,
        name: &str,
        description: &str,
        supply: u64,
        uri: &str,
        royalty_points_per_million: u64,
        property_keys: Vec<&str>,
        property_values: Vec<&str>,
        property_types: Vec<&str>,
    ) -> Result<String, ureq::Error> {
        let payload = TransactionPayload::EntryFunction(EntryFunction::new(
            ModuleId::new(
                AccountAddress::from_hex_literal("0x3").unwrap(),
                Identifier::new("token").unwrap(),
            ),
            Identifier::new("create_token_script").unwrap(),
            vec![],
            vec![
                bcs::to_bytes(collection_name).unwrap(),
                bcs::to_bytes(name).unwrap(),
                bcs::to_bytes(description).unwrap(),
                bcs::to_bytes(&supply).unwrap(),
                bcs::to_bytes(&supply).unwrap(),
                bcs::to_bytes(uri).unwrap(),
                bcs::to_bytes(&account.address()).unwrap(),
                bcs::to_bytes(&1000_000_u64).unwrap(),
                bcs::to_bytes(&royalty_points_per_million).unwrap(),
                bcs::to_bytes(&vec![false, false, false, false, false]).unwrap(),
                bcs::to_bytes(&property_keys).unwrap(),
                bcs::to_bytes(&property_values).unwrap(),
                bcs::to_bytes(&property_types).unwrap(),
            ],
        ));
        let signed_transaction = self
            .client
            .create_single_signer_bcs_transaction(account, payload);
        Ok(self.client.submit_bcs_transaction(signed_transaction)?)
    }

    // 发送 token
    pub fn offer_token(
        &self,
        account: LocalAccount,
        receiver: AccountAddress,
        creator: AccountAddress,
        collection_name: &str,
        token_name: &str,
        property_version: u64,
        amount: u64,
    ) -> Result<String, ureq::Error> {
        let payload = TransactionPayload::EntryFunction(EntryFunction::new(
            ModuleId::new(
                AccountAddress::from_hex_literal("0x3").unwrap(),
                Identifier::new("token_transfers").unwrap(),
            ),
            Identifier::new("offer_script").unwrap(),
            vec![],
            vec![
                bcs::to_bytes(&receiver.to_hex_literal()).unwrap(),
                bcs::to_bytes(&creator.to_hex_literal()).unwrap(),
                bcs::to_bytes(collection_name).unwrap(),
                bcs::to_bytes(token_name).unwrap(),
                bcs::to_bytes(&property_version).unwrap(),
                bcs::to_bytes(&amount).unwrap(),
            ],
        ));
        let signed_transaction = self
            .client
            .create_single_signer_bcs_transaction(account, payload);
        Ok(self.client.submit_bcs_transaction(signed_transaction)?)
    }

    // 索要 token
    pub fn claim_token(
        &self,
        account: LocalAccount,
        sender: AccountAddress,
        creator: AccountAddress,
        collection_name: &str,
        token_name: &str,
        property_version: u64,
    ) -> Result<String, ureq::Error> {
        let payload = TransactionPayload::EntryFunction(EntryFunction::new(
            ModuleId::new(
                AccountAddress::from_hex_literal("0x3").unwrap(),
                Identifier::new("token_transfers").unwrap(),
            ),
            Identifier::new("claim_script").unwrap(),
            vec![],
            vec![
                bcs::to_bytes(&sender.to_hex_literal()).unwrap(),
                bcs::to_bytes(&creator.to_hex_literal()).unwrap(),
                bcs::to_bytes(collection_name).unwrap(),
                bcs::to_bytes(token_name).unwrap(),
                bcs::to_bytes(&property_version).unwrap(),
            ],
        ));
        let signed_transaction = self
            .client
            .create_single_signer_bcs_transaction(account, payload);
        Ok(self.client.submit_bcs_transaction(signed_transaction)?)
    }

    // collection 数据
    pub fn collection_data(
        &self,
        account_address: AccountAddress,
        collection_name: &str,
    ) -> Result<CollectionData, ureq::Error> {
        Ok(self.client.table_item::<CollectionData>(
            &self.account_resource_collection_data_handle(account_address)?,
            "0x1::string::String",
            "0x3::token::CollectionData",
            serde_json::json!(collection_name),
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
        Ok(self.client.table_item::<Token>(
            &self.account_resource_tokens_handle(owner)?,
            "0x3::token::TokenId",
            "0x3::token::Token",
            token_id,
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
        Ok(self.client.table_item::<TokenData>(
            &self.account_resource_token_data_handle(creator)?,
            "0x3::token::TokenDataId",
            "0x3::token::TokenData",
            token_data_id,
            None,
        )?)
    }

    pub fn list_account_token_data(
        &self,
        account_address: AccountAddress,
        _start: u64,
        _limit: u64,
    ) -> Result<Vec<TokenData>, ureq::Error> {
        let events = self.client.events_by_event_handle::<EventData>(
            account_address,
            "0x3::token::TokenStore",
            "deposit_events",
            None,
            None,
        )?;
        let mut tokens = vec![];
        for e in events {
            let token_data_id = e.data.id.token_data_id;
            let token = self.token_data(
                // account_address,
                token_data_id.creator,
                &token_data_id.collection,
                &token_data_id.name,
                0,
            )?;
            tokens.push(token);
        }
        Ok(tokens)
    }

    pub fn token_balance(
        &self,
        owner: AccountAddress,
        creator: AccountAddress,
        collection_name: &str,
        token_name: &str,
        property_version: u64,
    ) -> Result<u64, ureq::Error> {
        let resp = self.token(
            owner,
            creator,
            collection_name,
            token_name,
            property_version,
        )?;
        Ok(resp.amount.0)
    }

    // 以下为私有方法
    fn account_resource_collection_data_handle(
        &self,
        account_address: AccountAddress,
    ) -> Result<String, ureq::Error> {
        Ok(self
            .client
            .account_resource::<ResourceCollectionData>(account_address, "0x3::token::Collections")?
            .data
            .collection_data
            .handle)
    }

    fn account_resource_token_data_handle(
        &self,
        account_address: AccountAddress,
    ) -> Result<String, ureq::Error> {
        Ok(self
            .client
            .account_resource::<ResourceTokenData>(account_address, "0x3::token::Collections")?
            .data
            .token_data
            .handle)
    }

    fn account_resource_tokens_handle(
        &self,
        account_address: AccountAddress,
    ) -> Result<String, ureq::Error> {
        Ok(self
            .client
            .account_resource::<ResourceTokens>(account_address, "0x3::token::TokenStore")?
            .data
            .tokens
            .handle)
    }
}

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

#[derive(Debug, Deserialize)]
pub struct EventData {
    pub amount: U64,
    pub id: EventId,
}

#[derive(Debug, Deserialize)]
pub struct EventId {
    pub token_data_id: TokenDataId,
}

// 以下私有方法使用
#[derive(Debug, Deserialize)]
struct ResourceHandle {
    pub handle: String,
}

#[derive(Debug, Deserialize)]
struct ResourceCollectionData {
    pub collection_data: ResourceHandle,
}

#[derive(Debug, Deserialize)]
struct ResourceTokenData {
    pub token_data: ResourceHandle,
}

#[derive(Debug, Deserialize)]
struct ResourceTokens {
    pub tokens: ResourceHandle,
}
