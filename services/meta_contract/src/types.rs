use marine_rs_sdk::marine;

#[marine]
pub struct MetaContractResult {
    pub result: bool,
    pub metadatas: Vec<FinalMetadata>,
    pub error_string: String,
}

#[marine]
pub struct FinalMetadata {
    pub public_key: String,
    pub alias: String,
    pub content: String,
}

#[marine]
#[derive(Debug, Clone)]
pub struct Metadata {
    pub hash: String,
    pub data_key: String,
    pub alias: String,
    pub cid: String,
    pub public_key: String,
}

#[marine]
#[derive(Debug, Clone)]
pub struct Transaction {
    pub hash: String,
    pub token_key: String,
    pub data_key: String,
    pub nonce: i64,
    pub from_peer_id: String,
    pub host_id: String,
    pub status: i64,
    pub data: String,
    pub public_key: String,
    pub alias: String,
    pub timestamp: u64,
    pub meta_contract_id: String,
    pub method: String,
    pub error_text: String,
    pub token_id: String,
    pub version: i64
}

#[marine]
#[derive(Debug, Default, Clone)]
pub struct MetaContract {
    pub token_key: String,
    pub meta_contract_id: String,
    pub public_key: String,
}

#[marine]
#[derive(Debug)]
pub struct TxParam {
    pub value_type: String,
    pub value: String,
}
