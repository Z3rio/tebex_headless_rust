//! A wrapper library for Tebex's Headless API, aka creating the serverside for your headless Tebex webstore.
//! 
//! # Installation
//! 
//! ```toml
//! tebex_headless_rust = "*"
//! ```
//! 
//! # Example Usage
//! ```rust
//! use tebex_headless_rust::{set_public_api_key, handlers::packages::{get_all_packages}};
//!
//! // tokio is used to allow an async main function
//! #[tokio::main]
//! async fn main() {
//!     // set public api key
//!     set_public_api_key(String::from("public_api_key_tebex"));
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
mod tests;
pub mod handlers;
pub mod models;

static BASE_URL: &str = "https://headless.tebex.io/api";