use ethabi::EventParam;
use marine_rs_sdk::marine;
use serde::{Deserialize, Serialize};
use serde_json::{Value};

#[marine]
#[derive(Debug, Serialize, Deserialize)]
pub struct EventLogParamResult {
    pub event_name: String,
    pub params: Vec<DataLogParam>,
    pub success: bool,
    pub error_msg: String,
    pub data: String,
    pub block_number: u64,
    pub transaction_hash: String,
}

#[marine]
#[derive(Debug, Serialize, Deserialize)]
pub struct DataLogParam {
    pub name: String,
    pub kind: String,
    pub value: String,
}

impl From<EventParam> for DataLogParam {
    fn from(param: EventParam) -> Self {
        Self {
            name: param.name,
            kind: param.kind.to_string(),
            value: "".to_string(),
        }
    }
}
