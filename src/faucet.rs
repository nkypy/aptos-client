use aptos_sdk::{
    bcs,
    types::{account_address::AccountAddress, transaction::SignedTransaction},
};
use std::time::Duration;

use crate::client::Client;

#[derive(Debug)]
pub struct FaucetClient {
    faucet_url: String,
    #[cfg(not(target_arch = "wasm32"))]
    inner: ureq::Agent,
    pub client: Client,
}

impl FaucetClient {
    pub fn new(faucet_url: &str, rest_url: &str) -> Self {
        Self {
            faucet_url: faucet_url.to_string(),
            #[cfg(not(target_arch = "wasm32"))]
            inner: ureq::AgentBuilder::new()
                .timeout_read(Duration::from_secs(2))
                .timeout_write(Duration::from_secs(2))
                .build(),
            client: Client::new(rest_url),
        }
    }

    pub fn create_account(&self, account_address: AccountAddress) -> Result<(), ureq::Error> {
        self.fund(account_address, 0)
    }

    pub fn fund(&self, account_address: AccountAddress, amount: u64) -> Result<(), ureq::Error> {
        let body = self
            .inner
            .post(&format!(
                "{}/mint?auth_key={}&amount={}&return_txns=true",
                self.faucet_url, account_address, amount
            ))
            .set("Content-Length", "0")
            .call()?
            .into_string()?;
        let bytes = hex::decode(body).unwrap();
        let txns: Vec<SignedTransaction> = bcs::from_bytes(&bytes).unwrap();
        self.client
            .wait_for_transaction(&txns[0].clone().committed_hash().to_hex());
        Ok(())
    }
}
