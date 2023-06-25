use marine_rs_sdk::marine;
use marine_rs_sdk::module_manifest;
use marine_rs_sdk::MountedBinaryResult;
use marine_rs_sdk::WasmLoggerBuilder;

mod bytes_type;
pub mod eth_calls;
pub mod eth_contract;
pub mod eth_utils;
mod fce_results;
mod jsonrpc_helpers;
mod models;
mod types;

module_manifest!();

pub fn main() {
    WasmLoggerBuilder::new()
        .with_log_level(log::LevelFilter::Info)
        .build()
        .unwrap();
}

pub fn curl_request_res(curl_cmd: Vec<String>) -> Result<String, std::string::FromUtf8Error> {
    log::info!("curl cmd: {:?}", curl_cmd);
    let response = curl(curl_cmd);
    let res = String::from_utf8(response.stdout)?;
    Ok(res)
}

#[marine]
#[link(wasm_import_module = "host")]
extern "C" {
    pub fn curl(curl_cmd: Vec<String>) -> MountedBinaryResult;
}
