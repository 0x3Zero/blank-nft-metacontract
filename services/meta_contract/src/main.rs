#![allow(improper_ctypes)]

mod data;
mod defaults;
mod types;

use std::collections::HashMap;
use data::{OpenSeaAttributes};
use defaults::*;
use marine_rs_sdk::marine;
use marine_rs_sdk::module_manifest;
use marine_rs_sdk::MountedBinaryResult;
use marine_rs_sdk::WasmLoggerBuilder;
use types::{MetaContract, Metadata, Transaction, TxParam, FinalMetadata, MetaContractResult};
use ethabi::{decode, ParamType};

module_manifest!();

pub fn main() {
    WasmLoggerBuilder::new()
        .with_log_level(log::LevelFilter::Info)
        .build()
        .unwrap();
}

#[marine]
pub fn on_execute(
    contract: MetaContract,
    metadatas: Vec<Metadata>,
    transaction: Transaction,
) -> MetaContractResult {
    let mut result = false;
    let mut error_string = "";

    let mut finals: Vec<FinalMetadata> = vec![];

    if !transaction.token_id.is_empty() && !transaction.alias.is_empty() {

      if is_token_owner(transaction.public_key.clone(), transaction.token_id.clone()) {
        finals.push(FinalMetadata {
          public_key: transaction.public_key,
          alias: transaction.alias,
          content: transaction.data,
        });

        result = true;
      } else {
        error_string = "Invalid token owner";
      }
    } else {
      error_string = "Missing token id or alias";
    }

    MetaContractResult {
      result,
      metadatas: finals,
      error_string: error_string.to_string(),
    }
}

#[marine]
pub fn on_clone() -> bool {
    return false;
}

#[marine]
pub fn on_mint(contract: MetaContract, data_key: String, token_id: String, data: String) -> MetaContractResult {
    let name = format!("Blank NFT #{}", token_id);
    let mut error: Option<String> = None;
    let mut finals: Vec<FinalMetadata> = vec![];

    if !token_id.is_empty() {
      let token_owner = get_token_owner(token_id.clone());

      finals.push(FinalMetadata {
        public_key: token_owner.clone(),
        alias: "name".to_string(),
        content: name,
      });
  
      finals.push(FinalMetadata {
          public_key: token_owner.clone(),
          alias: "description".to_string(),
          content: "Blank NFT description".to_string(),
      });
  
      finals.push(FinalMetadata {
          public_key: token_owner.clone(),
          alias: "image".to_string(),
          content: "ipfs://".to_string(),
      });
    } else {
      error = Some("Missing token id".to_string());
    }

    if !error.is_none() {
      return MetaContractResult {
        result: false,
        metadatas: Vec::new(),
        error_string: error.unwrap().to_string(),
      };
    }

    MetaContractResult {
        result: true,
        metadatas: finals,
        error_string: "".to_string(),
    }
}

fn get_token_owner(token_id: String) -> String {
  let mut tx_params: Vec<TxParam> = vec![];

  tx_params.push(TxParam {
    value_type: "uint".to_string(),
    value: token_id.clone()
  });

  let token_owner = evm_read_contract(
    DEFAULT_NODE_URL.to_string(), 
    DEFAULT_ABI_URL.to_string(), 
    "ownerOf".to_string(), 
    DEFAULT_CONTRACT_ADDRESS.to_string(),
    tx_params,
  );

  evm_shorten_hex(token_owner, 40)
}

fn is_token_owner(owner: String, token_id: String) -> bool {
  let token_owner = get_token_owner(token_id.clone());

  token_owner.to_lowercase() == owner.to_lowercase() 
}

// Service
// - curl

#[marine]
#[link(wasm_import_module = "host")]
extern "C" {
    pub fn ipfs(cmd: Vec<String>) -> MountedBinaryResult;
}

#[marine]
#[link(wasm_import_module = "evm_rpc")]
extern "C" {
    #[link_name = "contract_view_call"]
    pub fn evm_read_contract(
        node_url: String,
        abi_url: String,
        method_name: String,
        contract_address: String,
        tx_params: Vec<TxParam>,
    ) -> String;

    #[link_name = "shorten_hex"]
    pub fn evm_shorten_hex(hex: String, to_len: u32) -> String;
}
