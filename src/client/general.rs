use aptos_sdk::types::chain_id::ChainId;
use serde::Deserialize;

use crate::{move_types::U64, RestClient};

/// GET /
#[derive(Debug, Deserialize)]
pub struct LedgerInfo {
    /// Chain ID of the current chain
    pub chain_id: ChainId,
    // pub epoch: U64,
    pub ledger_version: U64,
    // pub oldest_ledger_version: U64,
    // pub block_height: U64,
    // pub oldest_block_height: U64,
    // pub ledger_timestamp: U64,
}

impl RestClient {
    // basic
    fn ledger_info(&self) -> Result<LedgerInfo, ureq::Error> {
        Ok(self
            .client
            .get(&self.base_url)
            .call()?
            .into_json::<LedgerInfo>()?)
    }

    // public
    pub fn chain_id(&self) -> Result<ChainId, ureq::Error> {
        Ok(self.ledger_info()?.chain_id)
    }
}
