use aptos_types::transaction::{RawTransaction, SignedTransaction, TransactionPayload};
use serde::Deserialize;
use std::{
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsValue;

use crate::types::{LocalAccount, U64};

impl super::Client {
    /// GET /transactions/by_hash/{txn_hash}
    pub fn transaction_by_hash(&self, txn_hash: &str) -> Result<Transaction, anyhow::Error> {
        #[cfg(not(target_arch = "wasm32"))]
        {
            Ok(self
                .inner
                .get(&format!(
                    "{}/transactions/by_hash/{}",
                    self.base_url, txn_hash
                ))
                .call()?
                .into_json::<Transaction>()?)
        }
        #[cfg(target_arch = "wasm32")]
        {
            Ok(self.web_request::<Transaction>(
                &format!("{}/transactions/by_hash/{}", self.base_url, txn_hash),
                "GET",
                None,
            )?)
        }
    }

    /// POST /transactions
    pub fn submit_bcs_transaction(
        &self,
        signed_transaction: SignedTransaction,
    ) -> Result<String, anyhow::Error> {
        let data = bcs::to_bytes(&signed_transaction).unwrap();
        #[cfg(not(target_arch = "wasm32"))]
        {
            Ok(self
                .inner
                .post(&format!("{}/transactions", self.base_url))
                .set("Content-Type", "application/x.aptos.signed_transaction+bcs")
                .send_bytes(&data)?
                .into_json::<SubmitTransaction>()?
                .hash)
        }
        #[cfg(target_arch = "wasm32")]
        {
            let body_array: js_sys::Uint8Array = data.into();
            let js_value: &JsValue = body_array.as_ref();
            Ok(self
                .web_request::<SubmitTransaction>(
                    &format!("{}/transactions", self.base_url),
                    "POST",
                    Some(js_value),
                )?
                .hash)
        }
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

    // 以下私有方法
    fn transaction_pending(&self, txn_hash: &str) -> bool {
        #[cfg(not(target_arch = "wasm32"))]
        {
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
        #[cfg(target_arch = "wasm32")]
        {
            if let Ok(txn) = self.web_request::<Transaction>(
                &format!("{}/transactions/by_hash/{}", self.base_url, txn_hash),
                "GET",
                None,
            ) {
                if txn.transaction_type != "pending_transaction".to_string() {
                    return false;
                }
            }
            false
        }
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
