//! This module contains all handlers for interacting with categories

use crate::{BASE_URL, models::{category::FullCategory, misc::Data}};

use super::misc::get_public_api_key;

/// Get category data based on the category id
/// If `include_package` is set to true, all packages of the category will also be fetched
/// Otherwise, an empty Vec of packages will be returned
pub async fn get_category(category_id: i32, include_packages: bool) -> Result<FullCategory, String> {
    let api_key = get_public_api_key();

    match api_key {
        Ok (api_key) => {
            let res = reqwest::get(format!("{0}/accounts/{1}/categories/{2}?includePackages={3}", BASE_URL, api_key, category_id, if include_packages { 1 } else { 0 }))
                .await;

            match res {
                Ok (data) => {
                    let json = data.json::<Data<FullCategory>>()
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

/// Gets a list of **all** categories in the webstore
/// If `include_package` is set to true, all packages of the category will also be fetched
/// Otherwise, an empty Vec of packages will be returned
pub async fn get_all_categories(include_packages: bool) -> Result<Vec<FullCategory>, String> {
    let api_key = get_public_api_key();

    match api_key {
        Ok (api_key) => {
            let res = reqwest::get(format!("{0}/accounts/{1}/categories?includePackages={2}", BASE_URL, api_key, if include_packages { 1 } else { 0 }))
                .await;

            match res {
                Ok (data) => {
                    let json = data.json::<Data<Vec<FullCategory>>>()
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