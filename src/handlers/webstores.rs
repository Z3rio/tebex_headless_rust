//! This module contains all the functions for interacting and fetching global information from your webstore

use crate::{BASE_URL, models::{webstores::Webstore, misc::Data}};

use super::misc::get_public_api_key;

/// Get all the webstore/misc data
pub async fn get_webstore() -> Result<Webstore, String> {
    let api_key = get_public_api_key();

    match api_key {
        Ok (api_key) => {
            let res = reqwest::get(format!("{0}/accounts/{1}", BASE_URL, api_key))
                .await;

            match res {
                Ok (data) => {
                    let json = data.json::<Data<Webstore>>()
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