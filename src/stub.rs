use cosmwasm_std::{Binary, ContractResult, SystemError, SystemResult};
pub struct Querier {}

impl cosmwasm_vm::Querier for Querier {
    fn query_raw(&self, request: &[u8], _: u64) -> cosmwasm_vm::BackendResult<SystemResult<ContractResult<Binary>>> {
        println!("{:?}", request);
        (serde_json::from_slice(b"{}").or_else(|_| {
            Ok(SystemResult::Err(SystemError::InvalidResponse {
                error: format!("ok"),
                response: cosmwasm_std::Binary::default()
            }))
        }), cosmwasm_vm::GasInfo::with_externally_used(0))
    }
}

pub struct Storage {}

impl cosmwasm_vm::Storage for Storage {
    fn get(&self, key: &[u8]) -> cosmwasm_vm::BackendResult<Option<Vec<u8>>> {
        println!("storage_get {:?} -> {:?}", key, std::str::from_utf8(&key));
        (Ok(Some(br#"{"count":"10","owner":"terra1x46rqay4d3cssq8gxxvqz8xt6nwlz4td20k38v"}"#.to_vec())), cosmwasm_vm::GasInfo::with_externally_used(0))
    }

    fn scan(&mut self, start: Option<&[u8]>, end: Option<&[u8]>, _order: cosmwasm_std::Order) -> cosmwasm_vm::BackendResult<u32> {
        println!("storage_scan {:?} & {:?}", start, end);
        (Ok(0), cosmwasm_vm::GasInfo::with_externally_used(0))
    }

    fn next(&mut self, iterator_id: u32) -> cosmwasm_vm::BackendResult<Option<cosmwasm_std::Pair>> {
        println!("storage_next {:?}", iterator_id);
        (Ok(None), cosmwasm_vm::GasInfo::with_externally_used(0))
    }

    // storage should be read-only
    fn set(&mut self, _: &[u8], _: &[u8]) -> cosmwasm_vm::BackendResult<()> {
        (Ok(()), cosmwasm_vm::GasInfo::with_externally_used(0))
    }
    
    // storage should be read-only
    fn remove(&mut self, _: &[u8]) -> cosmwasm_vm::BackendResult<()> {
        (Ok(()), cosmwasm_vm::GasInfo::with_externally_used(0))
    }
}