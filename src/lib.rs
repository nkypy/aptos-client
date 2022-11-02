use std::{
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use aptos_sdk::{
    bcs,
    move_types::{identifier::Identifier, language_storage::ModuleId},
    rest_client::aptos::Balance,
    types::{
        account_address::AccountAddress,
        chain_id::ChainId,
        transaction::{EntryFunction, RawTransaction, SignedTransaction, TransactionPayload},
        LocalAccount,
    },
};
use serde::{de::DeserializeOwned, Deserialize};

#[derive(Deserialize)]
pub struct Resource<T> {
    pub data: T,
}

#[derive(Deserialize)]
pub struct ChainInfo {
    pub chain_id: ChainId,
}

#[derive(Deserialize)]
pub struct AccountInfo {
    pub sequence_number: String,
}

#[derive(Deserialize)]
pub struct HashInfo {
    pub hash: String,
}

#[derive(Deserialize)]
pub struct TransactionInfo {
    #[serde(rename = "type")]
    pub hash_type: String,
}

#[derive(Debug, Deserialize)]
pub struct Collection {
    pub collection_data: Handle,
}

#[derive(Debug, Deserialize)]
pub struct TokenStore {
    pub tokens: Handle,
}

#[derive(Debug, Deserialize)]
pub struct TokenAmount {
    pub amount: String,
}

#[derive(Debug, Deserialize)]
pub struct TokenData {
    pub token_data: Handle,
}

#[derive(Debug, Deserialize)]
pub struct Handle {
    pub handle: String,
}

// 客户端

#[derive(Debug)]
pub struct RestClient {
    pub base_url: String,
    pub client: ureq::Agent,
    pub chain_id: ChainId,
}

impl RestClient {
    pub fn new(base_url: &str) -> Self {
        let mut client = Self {
            base_url: base_url.into(),
            client: ureq::AgentBuilder::new()
                .timeout_read(Duration::from_secs(2))
                .timeout_write(Duration::from_secs(2))
                .build(),
            chain_id: ChainId::test(),
        };
        client.chain_id = client.chain_id().unwrap();
        client
    }

    // ledger chain id
    fn chain_id(&self) -> Result<ChainId, ureq::Error> {
        Ok(self
            .client
            .get(&self.base_url)
            .call()?
            .into_json::<ChainInfo>()?
            .chain_id)
    }

    fn account_sequence_number(&self, account_address: AccountAddress) -> Result<u64, ureq::Error> {
        Ok(self
            .client
            .get(&format!("{}/accounts/{}", self.base_url, account_address))
            .call()?
            .into_json::<AccountInfo>()?
            .sequence_number
            .parse::<u64>()
            .unwrap())
    }

    pub fn account_resource<T: DeserializeOwned>(
        &self,
        account_address: AccountAddress,
        resource_type: &str,
    ) -> Result<T, ureq::Error> {
        Ok(self
            .client
            .get(&format!(
                "{}/accounts/{}/resource/{}",
                self.base_url, account_address, resource_type
            ))
            .call()?
            .into_json::<T>()?)
    }

    pub fn table_item(
        &self,
        table_handle: &str,
        key_type: &str,
        value_type: &str,
        key: serde_json::Value,
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

    pub fn create_single_signer_bcs_transaction(
        &self,
        sender: LocalAccount,
        payload: TransactionPayload,
    ) -> SignedTransaction {
        let txn = RawTransaction::new(
            sender.address(),
            self.account_sequence_number(sender.address()).unwrap(),
            payload,
            100_000,
            100,
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs()
                + 600,
            self.chain_id,
        );
        sender.sign_transaction(txn)
    }

    // 返回 hash
    pub fn submit_bcs_transaction(
        &self,
        signed_transaction: SignedTransaction,
    ) -> Result<String, ureq::Error> {
        let data = bcs::to_bytes(&signed_transaction).unwrap();
        Ok(self
            .client
            .post(&format!("{}/transactions", self.base_url))
            .set("Content-Type", "application/x.aptos.signed_transaction+bcs")
            .send_bytes(&data)?
            .into_json::<HashInfo>()?
            .hash)
    }

    fn transaction_pending(&self, txn_hash: &str) -> bool {
        if let Ok(resp) = self
            .client
            .get(&format!(
                "{}/transactions/by_hash/{}",
                self.base_url, txn_hash
            ))
            .call()
        {
            if resp.status() == 404 {
                return true;
            }
            if resp.into_json::<TransactionInfo>().unwrap().hash_type
                == "pending_transaction".to_string()
            {
                return true;
            }
        }
        return false;
    }

    pub fn wait_for_transaction(&self, txn_hash: &str) -> Result<(), ureq::Error> {
        let mut count = 0;
        while count <= 20 {
            if self.transaction_pending(txn_hash) {
                thread::sleep(Duration::from_secs(1));
                count += 1;
            } else {
                count += 20
            }
        }
        let _resp = self
            .client
            .get(&format!(
                "{}/transactions/by_hash/{}",
                self.base_url, txn_hash
            ))
            .call()?
            .into_string()?;
        Ok(())
    }

    // 完成，测试通过
    pub fn account_balance(&self, account_address: AccountAddress) -> Result<u64, ureq::Error> {
        Ok(self
            .account_resource::<Resource<Balance>>(
                account_address,
                "0x1::coin::CoinStore<0x1::aptos_coin::AptosCoin>",
            )?
            .data
            .coin
            .value
            .0)
    }

    // token 相关
    pub fn collection(
        &self,
        account_address: AccountAddress,
        collection_name: &str,
    ) -> Result<String, ureq::Error> {
        let table_handle = self
            .account_resource::<Resource<Collection>>(account_address, "0x3::token::Collections")?
            .data
            .collection_data
            .handle;
        Ok(self.table_item(
            &table_handle,
            "0x1::string::String",
            "0x3::token::CollectionData",
            serde_json::json!(collection_name),
        )?)
    }

    // 完成，测试通过
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
        let signed_transaction = self.create_single_signer_bcs_transaction(account, payload);
        Ok(self.submit_bcs_transaction(signed_transaction)?)
    }

    pub fn token(
        &self,
        owner: AccountAddress,
        creater: AccountAddress,
        collection_name: &str,
        token_name: &str,
        property_version: u64,
    ) -> Result<String, ureq::Error> {
        let table_handle = self
            .account_resource::<Resource<TokenStore>>(owner, "0x3::token::TokenStore")?
            .data
            .tokens
            .handle;
        let token_id = serde_json::json!({
            "token_data_id": {
                "creator": creater.to_hex_literal(),
                "collection": collection_name,
                "name": token_name,
            },
            "property_version": format!("{}", property_version),
        });
        Ok(self.table_item(
            &table_handle,
            "0x3::token::TokenId",
            "0x3::token::Token",
            token_id,
        )?)
    }

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
        let signed_transaction = self.create_single_signer_bcs_transaction(account, payload);
        Ok(self.submit_bcs_transaction(signed_transaction)?)
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
        let amount: TokenAmount = serde_json::from_str(&resp).unwrap();
        Ok(amount.amount.parse::<u64>().unwrap())
    }

    pub fn token_data(
        &self,
        creator: AccountAddress,
        collection_name: &str,
        token_name: &str,
        _property_version: u64,
    ) -> Result<String, ureq::Error> {
        let table_handle = self
            .account_resource::<Resource<TokenData>>(creator, "0x3::token::Collections")?
            .data
            .token_data
            .handle;

        let token_data_id = serde_json::json!({
            "creator": creator.to_hex_literal(),
            "collection": collection_name,
            "name": token_name,
        });
        Ok(self.table_item(
            &table_handle,
            "0x3::token::TokenDataId",
            "0x3::token::TokenData",
            token_data_id,
        )?)
    }

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
        let signed_transaction = self.create_single_signer_bcs_transaction(account, payload);
        Ok(self.submit_bcs_transaction(signed_transaction)?)
    }

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
        let signed_transaction = self.create_single_signer_bcs_transaction(account, payload);
        Ok(self.submit_bcs_transaction(signed_transaction)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
