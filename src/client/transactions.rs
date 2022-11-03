use aptos_sdk::{bcs, types::transaction::SignedTransaction};
use serde::Deserialize;

use crate::{move_types::U64, RestClient};

#[derive(Debug, Deserialize)]
pub struct Transaction {
    #[serde(rename = "type")]
    pub transaction_type: String,
    pub sequence_number: U64,
}

#[derive(Deserialize)]
pub struct SubmitTransactionResponse {
    pub hash: String,
}

impl RestClient {
    // private
    fn create_transaction_payload(&self) {}

    // public

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
            .into_json::<SubmitTransactionResponse>()?
            .hash)
    }
}
