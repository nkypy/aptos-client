use aptos_types::account_address::AccountAddress;
use serde::{de::DeserializeOwned, Deserialize};

use crate::types::U64;

impl super::Client {
    /// GET /accounts/{address}
    pub fn account(
        &self,
        account_address: AccountAddress,
        _ledger_version: Option<U64>,
    ) -> Result<Account, anyhow::Error> {
        #[cfg(not(target_arch = "wasm32"))]
        {
            Ok(self
                .inner
                .get(&format!("{}/accounts/{}", self.base_url, account_address))
                .call()?
                .into_json::<Account>()?)
        }
        #[cfg(target_arch = "wasm32")]
        {
            Ok(self.web_request::<Account>(
                &format!("{}/accounts/{}", self.base_url, account_address),
                "GET",
                None,
            )?)
        }
    }

    /// GET /accounts/{address}/resource/{resource_type}
    pub fn account_resource<T: DeserializeOwned>(
        &self,
        account_address: AccountAddress,
        resource_type: &str,
    ) -> Result<AccountResource<T>, anyhow::Error> {
        #[cfg(not(target_arch = "wasm32"))]
        {
            Ok(self
                .inner
                .get(&format!(
                    "{}/accounts/{}/resource/{}",
                    self.base_url, account_address, resource_type
                ))
                .call()?
                .into_json::<AccountResource<T>>()?)
        }
        #[cfg(target_arch = "wasm32")]
        {
            Ok(self.web_request::<AccountResource<T>>(
                &format!(
                    "{}/accounts/{}/resource/{}",
                    self.base_url, account_address, resource_type
                ),
                "GET",
                None,
            )?)
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Account {
    pub sequence_number: U64,
}

#[derive(Debug, Deserialize)]
pub struct AccountResource<T> {
    #[serde(rename = "type")]
    pub resource_type: String,
    pub data: T,
}
