#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;

#[global_allocator]
static ALLOC: mini_alloc::MiniAlloc = mini_alloc::MiniAlloc::INIT;

use alloy_sol_types::sol;
use stylus_sdk::{abi::Bytes, alloy_primitives::Address, call::RawCall, prelude::*};

#[solidity_storage]
#[entrypoint]
pub struct MultiCall;

// Declare events and Solidity error types
sol! {
    error ArraySizeNotMatch();
    error CallFailed();
}

#[derive(SolidityError)]
pub enum MulticallErrors {
    ArraySizeNotMatch(ArraySizeNotMatch),
    CallFailed(CallFailed),
}

#[external]
impl MultiCall {
    pub fn multicall(
        &self,
        addresses: Vec<Address>,
        data: Vec<Bytes>,
    ) -> Result<Vec<Bytes>, MulticallErrors> {
        let addr_len = addresses.len();
        let data_len = data.len();
        let mut results: Vec<Bytes> = Vec::new();
        if addr_len != data_len {
            return Err(MulticallErrors::ArraySizeNotMatch(ArraySizeNotMatch {}));
        }
        for i in 0..addr_len {
            let result: Result<Vec<u8>, Vec<u8>> =
                RawCall::new().call(addresses[i], data[i].to_vec().as_slice());
            let data = match result {
                Ok(data) => data,
                Err(_data) => return Err(MulticallErrors::CallFailed(CallFailed {})),
            };
            results.push(data.into())
        }
        Ok(results)
    }
}
