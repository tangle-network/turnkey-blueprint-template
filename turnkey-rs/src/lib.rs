mod bytes;

pub mod client;
pub mod errors;
pub mod functions;
pub mod utils;

pub use client::Turnkey;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiStamp {
    pub public_key: String,
    pub signature: String,
    pub scheme: &'static str,
}
