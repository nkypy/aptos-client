use serde::de::DeserializeOwned;

use crate::types::U64;

impl super::Client {
    /// POST /tables/{table_handle}/item
    pub fn table_item<T: DeserializeOwned>(
        &self,
        table_handle: &str,
        key_type: &str,
        value_type: &str,
        key: serde_json::Value,
        _ledger_version: Option<U64>,
    ) -> Result<T, anyhow::Error> {
        #[cfg(not(target_arch = "wasm32"))]
        {
            Ok(self
                .inner
                .post(&format!("{}/tables/{}/item", self.base_url, table_handle))
                .send_json(serde_json::json!({
                    "key_type": key_type,
                    "value_type": value_type,
                    "key": key,
                }))?
                .into_json::<T>()?)
        }
        #[cfg(target_arch = "wasm32")]
        {
            Ok(self.fetch::<T>(
                &format!("{}/tables/{}/item", self.base_url, table_handle),
                "POST",
                Some(
                    &serde_wasm_bindgen::to_value(&serde_json::json!({
                        "key_type": key_type,
                        "value_type": value_type,
                        "key": key,
                    }))
                    .unwrap(),
                ),
            )?)
        }
    }
}
