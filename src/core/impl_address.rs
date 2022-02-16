use std::borrow::Borrow;
use crate::core::{CoreAddress, CoreImpl};
use crate::core::address::Address;
use jsonrpc_core::Result;

impl CoreAddress for CoreImpl {
    fn create_address(&self, public_key: Option<Box<[u8]>>) -> Result<Address> {
        match public_key {
            Some(pubK) => Ok(Address::from_public(pubK.borrow())),
            None => Ok(Address::create())
        }
    }
}