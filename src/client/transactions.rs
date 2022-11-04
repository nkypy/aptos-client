use aptos_sdk::{
    bcs,
    types::{
        transaction::{RawTransaction, SignedTransaction, TransactionPayload},
        LocalAccount,
    },
};
use serde::Deserialize;
use std::{
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use crate::types::U64;

impl super::Client {
    /// GET /transactions/by_hash/{txn_hash}
    pub fn transaction_by_hash(&self, txn_hash: &str) -> Result<Transaction, ureq::Error> {
        Ok(self
            .inner
            .get(&format!(
                "{}/transactions/by_hash/{}",
                self.base_url, txn_hash
            ))
            .call()?
            .into_json::<Transaction>()?)
    }

    /// POST /transactions
    pub fn submit_bcs_transaction(
        &self,
        signed_transaction: SignedTransaction,
    ) -> Result<String, ureq::Error> {
        let data = bcs::to_bytes(&signed_transaction).unwrap();
        Ok(self
            .inner
            .post(&format!("{}/transactions", self.base_url))
            .set("Content-Type", "application/x.aptos.signed_transaction+bcs")
            .send_bytes(&data)?
            .into_json::<SubmitTransaction>()?
            .hash)
    }

    // single signer sign transaction
    pub fn create_single_signer_bcs_transaction(
        &self,
        sender: LocalAccount,
        payload: TransactionPayload,
    ) -> SignedTransaction {
        let txn = RawTransaction::new(
            sender.address(),
            self.account(sender.address(), None)
                .unwrap()
                .sequence_number
                .0,
            payload,
            100_000,
            100,
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs()
                + 600,
            self.ledger_info().unwrap().chain_id,
        );
        sender.sign_transaction(txn)
    }

    // 等待交易完成
    pub fn wait_for_transaction(&self, txn_hash: &str) -> () {
        let mut count = 0;
        while count <= 10 {
            if self.transaction_pending(txn_hash) {
                thread::sleep(Duration::from_secs(1));
                count += 1;
            } else {
                count += 10
            }
        }
        ()
    }

    // 一下私有方法
    fn transaction_pending(&self, txn_hash: &str) -> bool {
        if let Ok(resp) = self
            .inner
            .get(&format!(
                "{}/transactions/by_hash/{}",
                self.base_url, txn_hash
            ))
            .call()
        {
            if let Ok(txn) = resp.into_json::<Transaction>() {
                if txn.transaction_type != "pending_transaction".to_string() {
                    return false;
                }
            }
        }
        true
    }
}

#[derive(Debug, Deserialize)]
pub struct Transaction {
    #[serde(rename = "type")]
    pub transaction_type: String,
    pub sequence_number: U64,
}

#[derive(Deserialize)]
pub struct SubmitTransaction {
    pub hash: String,
}
