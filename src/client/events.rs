use aptos_types::account_address::AccountAddress;
use serde::{de::DeserializeOwned, Deserialize};

use crate::types::U64;

impl super::Client {
    /// GET /accounts/{address}/events/{event_handle}/{field_name}
    pub fn events_by_event_handle<T: DeserializeOwned>(
        &self,
        account_address: AccountAddress,
        event_handle: &str,
        field_name: &str,
        _limit: Option<u64>,
        _start: Option<U64>,
    ) -> Result<Vec<Event<T>>, anyhow::Error> {
        #[cfg(not(target_arch = "wasm32"))]
        {
            Ok(self
                .inner
                .get(&format!(
                    "{}/accounts/{}/events/{}/{}",
                    self.base_url,
                    account_address.to_hex_literal(),
                    event_handle,
                    field_name
                ))
                .call()?
                .into_json::<Vec<Event<T>>>()?)
        }
        #[cfg(target_arch = "wasm32")]
        {
            Ok(self.web_request::<Vec<Event<T>>>(
                &format!(
                    "{}/accounts/{}/events/{}/{}",
                    self.base_url,
                    account_address.to_hex_literal(),
                    event_handle,
                    field_name
                ),
                "GET",
                None,
            )?)
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Event<T> {
    #[serde(rename = "type")]
    pub event_type: String,
    pub sequence_number: U64,
    pub data: T,
}
