use serde::de::DeserializeOwned;
use serde_json::Result as JsonResult;

pub use serde_json::to_vec;

pub fn from_bytes<T: DeserializeOwned>(bytes: &[u8]) -> JsonResult<T> {
    serde_json::from_slice(bytes)
}
