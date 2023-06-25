use crate::curl_request_res;
use crate::eth_contract::{decode_batch_logs, decode_logs};
use crate::eth_utils::{
    check_response_block_string, check_response_log_string, check_response_string,
    check_response_transaction_string, get_nonce,
};
use crate::fce_results::{
    JsonRpcBlockResult, JsonRpcLogResult, JsonRpcResult, JsonRpcTransactionResult,
};
use crate::jsonrpc_helpers::Request;
use crate::models::log_param::EventLogParamResult;
use crate::types::TxCall;

use jsonrpc_core as rpc;
use marine_rs_sdk::marine;
use serde_json::json;

pub fn serialize<T: serde::Serialize>(t: &T) -> rpc::Value {
    serde_json::to_value(t).expect("Types never fail to serialize.")
}

pub fn eth_call(url: String, tx: TxCall, tag: String) -> JsonRpcResult {
    let method = "eth_call".to_string();

    let tx_call_serial = serialize(&tx);
    let tag_serial = serialize(&tag);
    let params: rpc::Value = json!(vec![tx_call_serial, tag_serial]);

    let id = get_nonce();

    let curl_args = Request::new(method, params, id).as_sys_string(&url);
    let response = curl_request_res(curl_args).unwrap();

    check_response_string(response, &id)
}

// pub fn eth_send_transaction(url: String, tx: TxCall) -> JsonRpcResult {
//     let method = "eth_sendTransaction".to_string();

//     let tx_call_serial = serialize(&tx);
//     let params: rpc::Value = json!(vec![tx_call_serial]);

//     let id = get_nonce();

//     let curl_args = Request::new(method, params, id).as_sys_string(&url);
//     let response = curl_request_res(curl_args).unwrap();

//     check_response_string(response, &id)
// }

#[marine]
pub fn eth_get_transaction_receipt(url: String, trans_hash: String) -> JsonRpcTransactionResult {
    let method = "eth_getTransactionReceipt".to_string();

    let trans_hash_serial = serialize(&trans_hash);
    log::info!("{}", trans_hash_serial);

    let params: rpc::Value = json!(vec![trans_hash_serial]);

    let id = get_nonce();

    let curl_args = Request::new(method, params, id).as_sys_string(&url);
    let response = curl_request_res(curl_args).unwrap();
    log::info!("{:?}", response);
    check_response_transaction_string(response, &id)
}

#[marine]
pub fn eth_get_latest_block_number(url: String) -> JsonRpcResult {
    let method = "eth_blockNumber".to_string();
    let params: rpc::Value = json!([]);

    let id = get_nonce();

    let curl_args = Request::new(method, params, id).as_sys_string(&url);
    log::info!("{}", curl_args.join(" "));
    let response = curl_request_res(curl_args).unwrap();

    log::info!("{}", response);
    check_response_string(response, &id)
}

#[marine]
pub fn eth_get_block_by_number(url: String, block_in_hex: String) -> JsonRpcBlockResult {
    let method = "eth_getBlockByNumber".to_string();

    let block_serial = serialize(&block_in_hex);
    let is_hydrated_serial = serialize(&true);
    let params: rpc::Value = json!(vec![block_serial, is_hydrated_serial]);

    let id = get_nonce();

    let curl_args = Request::new(method, params, id).as_sys_string(&url);
    let response = curl_request_res(curl_args).unwrap();

    check_response_block_string(response, &id)
}

#[marine]
pub fn eth_send_raw_transaction(url: String, signed_tx: String) -> JsonRpcResult {
    let method = "eth_sendRawTransaction".to_string();

    let signed_tx_serial = serialize(&signed_tx);
    let params: rpc::Value = json!(vec![signed_tx_serial]);

    let id = get_nonce();

    let curl_args = Request::new(method, params, id).as_sys_string(&url);
    let response = curl_request_res(curl_args).unwrap();

    check_response_string(response, &id)
}

#[marine]
pub fn eth_get_balance(url: String, add: String) -> JsonRpcResult {
    let method = "eth_getBalance".to_string();

    let add_serial = serialize(&add);
    let tag_serial = serialize(&"latest".to_string());
    let params: rpc::Value = json!(vec![add_serial, tag_serial]);

    let id = get_nonce();

    let curl_args = Request::new(method, params, id).as_sys_string(&url);
    let response = curl_request_res(curl_args).unwrap();

    log::info!("{}", response);
    check_response_string(response, &id)
}

#[marine]
pub fn eth_get_logs(
    url: String,
    abi_url: String,
    start_block_in_hex: &str,
    end_block_in_hex: &str,
    address: &str,
    topics: Vec<String>,
) -> Vec<EventLogParamResult> {
    let method = "eth_getLogs".to_string();

    let filter = json!({
        "fromBlock": start_block_in_hex,
        "toBlock": end_block_in_hex,
        "address": address,
        "topics": topics
    });

    let params: rpc::Value = json!(vec![filter]);

    let id = get_nonce();

    let curl_args = Request::new(method, params, id).as_sys_string(&url);
    let response = curl_request_res(curl_args).unwrap();

    let log_result = check_response_log_string(response, &id);

    decode_batch_logs(abi_url, log_result.clone().result)
}
