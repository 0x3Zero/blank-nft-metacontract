use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct OpenSeaAttributes {
  pub display_type: String,
  pub trait_type: String,
  pub value: i32,
}