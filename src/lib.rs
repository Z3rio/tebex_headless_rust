//! A wrapper library for Tebex's Headless API, aka creating the serverside for your headless Tebex webstore.
//! 
//! # Installation
//! 
//! ```toml
//! tebex_headless_rust = "*"
//! ```
//! 
//! # Example Usage
//! ```rs
//! use tebex_headless_rust::{set_public_api_key, handlers::packages::{get_all_packages}};
//! 
//! async fn main() {
//!     // set public api key
//!     set_public_api_key("public_api_key_tebex");
//! 
//!     // fetch packages
//!     let packages = get_all_packages().await;
//! 
//!     match packages {
//!         // if packages successfully fetched
//!         Ok (packages) => {
//!             println!("Package amount: {}", packages.len());
//!         }
//! 
//!         // handle issue with fetching of packages
//!         Err (err) => {
//!             println!("Could not fetch pacakges: {}", err);
//!         }
//!     }
//! }
//! ```

#![warn(
    missing_debug_implementations,
    missing_docs,
    rust_2018_idioms,
    unreachable_pub
)]
#![doc(test(
    no_crate_inject,
    attr(deny(warnings, rust_2018_idioms), allow(dead_code, unused_variables))
))]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, allow(unused_attributes))]

#[doc(hidden)]
pub mod models;
#[doc(hidden)]
pub mod tests;

#[doc(hidden)]
pub mod handlers;

use std::env::VarError;
use dotenv::dotenv;

static BASE_URL: &str = "https://headless.tebex.io/api";

/// Sets the public api key
pub fn set_public_api_key(api_key: String) {
    std::env::set_var("PUBLIC_API_KEY", api_key);
}

/// Gets the public api key
pub fn get_public_api_key() -> Result<String, String> {
    let res = get_env_var("TEBEX_PUBLIC_KEY");

    match res {
        Ok(res_data) => {
            return Ok(res_data)
        }

        _ => {
            return Err(String::from("Public API key is not set"));
        }
    }
}

fn get_env_var(query: &str) -> Result<String, VarError> {
    if cfg!(test) && !dotenv().is_ok() {
        dotenv().ok();
    }

    let queried: Result<String, VarError> = std::env::var(query);

    return queried;
}