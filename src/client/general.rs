use aptos_types::chain_id::ChainId;
use serde::Deserialize;

use crate::types::U64;

impl super::Client {
    /// GET /
    pub fn ledger_info(&self) -> Result<LedgerInfo, anyhow::Error> {
        #[cfg(not(target_arch = "wasm32"))]
        {
            Ok(self
                .inner
                .get(&self.base_url)
                .call()?
                .into_json::<LedgerInfo>()?)
        }

        #[cfg(target_arch = "wasm32")]
        {
            Ok(self.fetch::<LedgerInfo>(&self.base_url, "GET", None)?)
        }
    }
}

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
