//! This module contains all the functions which could not be categorized into any of the previous categories

use std::env::VarError;
use dotenv::dotenv;

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

#[doc(hidden)]
pub fn get_env_var(query: &str) -> Result<String, VarError> {
    if cfg!(test) && !dotenv().is_ok() {
        dotenv().ok();
    }

    let queried: Result<String, VarError> = std::env::var(query);

    return queried;
}