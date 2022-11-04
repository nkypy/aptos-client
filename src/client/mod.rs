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

use std::time::Duration;

#[derive(Debug, Clone)]
pub struct Client {
    base_url: String,
    #[cfg(not(target_arch = "wasm32"))]
    inner: ureq::Agent,
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
        }
    }
}
