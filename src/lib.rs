pub mod client;
pub mod move_types;

use std::{
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use aptos_sdk::{
    bcs,
    move_types::{identifier::Identifier, language_storage::ModuleId},
    types::{
        account_address::AccountAddress,
        chain_id::ChainId,
        transaction::{EntryFunction, RawTransaction, SignedTransaction, TransactionPayload},
        LocalAccount,
    },
};

use crate::client::{EventData, TokenData, Transaction};

// 客户端

#[derive(Debug)]
pub struct RestClient {
    pub base_url: String,
    #[cfg(not(target_arch = "wasm32"))]
    pub client: ureq::Agent,
    pub chain_id: ChainId,
}

impl RestClient {
    pub fn new(base_url: &str) -> Self {
        let mut client = Self {
            base_url: base_url.into(),
            #[cfg(not(target_arch = "wasm32"))]
            client: ureq::AgentBuilder::new()
                .timeout_read(Duration::from_secs(2))
                .timeout_write(Duration::from_secs(2))
                .build(),
            chain_id: ChainId::test(),
        };
        client.chain_id = client.chain_id().unwrap();
        client
    }

    pub fn create_single_signer_bcs_transaction(
        &self,
        sender: LocalAccount,
        payload: TransactionPayload,
    ) -> SignedTransaction {
        let txn = RawTransaction::new(
            sender.address(),
            self.account_sequence_number(sender.address(), None)
                .unwrap(),
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
            if resp.into_json::<Transaction>().unwrap().transaction_type
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
        Ok(resp.amount.0)
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

    pub fn list_account_token_data(
        &self,
        account_address: AccountAddress,
        _start: u64,
        _limit: u64,
    ) -> Result<Vec<TokenData>, ureq::Error> {
        let events = self.events_by_event_handle::<EventData>(
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
