use aptos_sdk::types::account_address::AccountAddress;
use serde::{de::DeserializeOwned, Deserialize};

use crate::{move_types::U64, RestClient};

/// GET /accounts/{address}/events/{event_handle}/{field_name}
#[derive(Debug, Deserialize)]
pub struct Event<T> {
    #[serde(rename = "type")]
    pub event_type: String,
    pub sequence_number: U64,
    pub data: T,
}

#[derive(Debug, Deserialize)]
pub struct EventData {
    pub amount: U64,
    pub id: EventId,
}

#[derive(Debug, Deserialize)]
pub struct EventId {
    pub token_data_id: TokenDataId,
}

#[derive(Debug, Deserialize)]
pub struct TokenDataId {
    pub creator: AccountAddress,
    pub collection: String,
    pub name: String,
}

impl RestClient {
    pub fn events_by_event_handle<T: DeserializeOwned>(
        &self,
        account_address: AccountAddress,
        event_handle: &str,
        field_name: &str,
        _limit: Option<u64>,
        _start: Option<U64>,
    ) -> Result<Vec<Event<T>>, ureq::Error> {
        Ok(self
            .client
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
}
