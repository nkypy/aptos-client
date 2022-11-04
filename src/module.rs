use aptos_sdk::{
    move_types::{
        identifier::Identifier,
        language_storage::{ModuleId, TypeTag},
    },
    types::{
        account_address::AccountAddress,
        transaction::{EntryFunction, TransactionPayload},
        LocalAccount,
    },
};

use crate::client::Client;

#[derive(Debug)]
pub struct ModuleClient {
    module: ModuleId,
    pub client: Client,
}

impl ModuleClient {
    pub fn new(rest_url: &str, address: AccountAddress, name: &str) -> Self {
        Self {
            module: ModuleId::new(address, Identifier::new(name).unwrap()),
            client: Client::new(rest_url),
        }
    }

    pub fn entry_function(
        &self,
        account: LocalAccount,
        function: &str,
        ty_args: Vec<TypeTag>,
        args: Vec<Vec<u8>>,
    ) -> Result<String, ureq::Error> {
        let payload = TransactionPayload::EntryFunction(EntryFunction::new(
            self.module.clone(),
            Identifier::new(function).unwrap(),
            ty_args,
            args,
        ));
        let signed_transaction = self
            .client
            .create_single_signer_bcs_transaction(account, payload);
        Ok(self.client.submit_bcs_transaction(signed_transaction)?)
    }
}
