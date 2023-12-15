//! This module contains all handlers for interacting with packages

use crate::{BASE_URL, models::{package::Package, misc::Data}};

use super::misc::get_public_api_key;

/// Get package data based on the packages id
pub async fn get_package(package_id: i32, basket_identifier: Option<String>, ip_address: Option<String>) -> Result<Package, String> {
    let api_key = get_public_api_key();

    match api_key {
        Ok (api_key) => {
            let res = reqwest::get(format!(
                "{0}/accounts/{1}/packages/{2}{3}{4}",
                BASE_URL,
                api_key,
                package_id,
                if basket_identifier.is_none() { String::from("") } else { format!("?basketIdent={0}", basket_identifier.clone().unwrap()) },
                if ip_address.is_none() { String::from("") } else { format!("{0}ipAddress={1}", 
                    if basket_identifier.is_none() { String::from("?") } else { String::from("&") },
                    ip_address.unwrap()
                )}
            ))
                .await;

            match res {
                Ok (data) => {
                    let json = data.json::<Data<Package>>()
                        .await;

                    match json {
                        Ok (json_data) => {
                            return Ok(json_data.data);
                        }

                        Err(err) => {
                            return Err(String::from(format!("Error occured whilst parsing JSON, {0}", err)))
                        }
                    }
                }

                _ => {
                    return Err(String::from("Unknown error occured"));
                }
            }
        }

        _ => {
            return Err(String::from("Unknown error occured"));
        }
    }
}

/// Get a list/vector of all packages
pub async fn get_all_packages(basket_identifier: Option<String>, ip_address: Option<String>) -> Result<Vec<Package>, String> {
    let api_key = get_public_api_key();

    match api_key {
        Ok (api_key) => {
            let res = reqwest::get(format!(
                "{0}/accounts/{1}/packages{2}{3}",
                BASE_URL,
                api_key,
                if basket_identifier.is_none() { String::from("") } else { format!("?basketIdent={0}", basket_identifier.clone().unwrap()) },
                if ip_address.is_none() { String::from("") } else { format!("{0}ipAddress={1}", 
                    if basket_identifier.is_none() { String::from("?") } else { String::from("&") },
                    ip_address.unwrap()
                )}
            ))
                .await;

            match res {
                Ok (data) => {
                    let json = data.json::<Data<Vec<Package>>>()
                        .await;

                    match json {
                        Ok (json_data) => {
                            return Ok(json_data.data);
                        }

                        Err(err) => {
                            return Err(String::from(format!("Error occured whilst parsing JSON, {0}", err)))
                        }
                    }
                }

                _ => {
                    return Err(String::from("Unknown error occured"));
                }
            }
        }

        _ => {
            return Err(String::from("Unknown error occured"));
        }
    }
}