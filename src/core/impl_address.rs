use std::borrow::Borrow;
use crate::core::address::Address;
use jsonrpc_core::Result;

pub struct CoreAddressImpl;
use jsonrpc_derive::rpc;

#[rpc]
pub trait CoreAddress {
    #[rpc(name = "create_address")]
    fn create_address(&self, public_key: Option<Box<[u8]>>) -> Result<Address>;
}

impl CoreAddress for CoreAddressImpl {
    fn create_address(&self, public_key: Option<Box<[u8]>>) -> Result<Address> {
        match public_key {
            Some(pubK) => Ok(Address::from_public(pubK.borrow())),
            None => Ok(Address::create())
        }
    }
}