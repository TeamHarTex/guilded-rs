use serde::de::DeserializeOwned;
pub use serde_json::to_vec;
use serde_json::Result as JsonResult;

pub fn from_bytes<T: DeserializeOwned>(bytes: &[u8]) -> JsonResult<T> {
    serde_json::from_slice(bytes)
}
