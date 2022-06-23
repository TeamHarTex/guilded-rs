//! # guilded_http
//!
//! Guilded API HTTP Client for the Guilded-rs ecosystem of crates.

pub mod client;
pub mod error;
pub mod json;
pub mod request;
pub mod response;
pub mod route;

pub const API_VERSION: u8 = 10;
