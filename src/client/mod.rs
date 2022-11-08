mod accounts;
mod blocks;
mod events;
mod general;
mod tables;
mod transactions;

pub use accounts::*;
pub use blocks::*;
pub use events::*;
pub use general::*;
pub use tables::*;
pub use transactions::*;

#[cfg(target_arch = "wasm32")]
use serde::de::DeserializeOwned;
use std::time::Duration;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::{prelude::*, JsCast};

#[derive(Debug, Clone)]
pub struct Client {
    base_url: String,
    #[cfg(not(target_arch = "wasm32"))]
    inner: ureq::Agent,
    #[cfg(target_arch = "wasm32")]
    inner: web_sys::Window,
}

impl Client {
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.into(),
            #[cfg(not(target_arch = "wasm32"))]
            inner: ureq::AgentBuilder::new()
                .timeout_read(Duration::from_secs(2))
                .timeout_write(Duration::from_secs(2))
                .build(),
            #[cfg(target_arch = "wasm32")]
            inner: web_sys::window().unwrap(),
        }
    }
    #[cfg(target_arch = "wasm32")]
    async fn async_web_request<T: DeserializeOwned>(
        &self,
        url: &str,
        method: &str,
        body: Option<&JsValue>,
    ) -> Result<T, JsValue> {
        let mut opts = web_sys::RequestInit::new();
        opts.method(method);
        opts.body(body);
        let request = web_sys::Request::new_with_str_and_init(url, &opts).unwrap();
        if body != None {
            request.headers().set("Content-Type", "application/x.aptos.signed_transaction+bcs");
        }
        let resp_value =
            wasm_bindgen_futures::JsFuture::from(self.inner.fetch_with_request(&request)).await?;
        let resp: web_sys::Response = resp_value.dyn_into().unwrap();
        let json = wasm_bindgen_futures::JsFuture::from(resp.json()?).await?;
        let val: T = serde_wasm_bindgen::from_value(json)?;
        Ok(val)
    }
    #[cfg(target_arch = "wasm32")]
    fn web_request<T: DeserializeOwned>(
        &self,
        url: &str,
        method: &str,
        body: Option<JsValue>,
    ) -> Result<T, JsValue> {
        wasm_bindgen_futures::spawn_local(self.async_web_request(url, method, body))
    }
}
