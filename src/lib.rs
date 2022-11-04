pub mod types;

mod client;
mod coin;
mod faucet;
mod module;
mod token;

pub use client::Client;
pub use coin::CoinClient;
pub use faucet::FaucetClient;
pub use module::ModuleClient;
pub use token::TokenClient;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
