//! This module contains the functions for adding/removing creator codes

use serde_json::json;

use crate::{BASE_URL, models::misc::Message};

use super::misc::get_public_api_key;

/// Apply a creator code to a specific basket
pub async fn apply_creator_code(basket_identifier: String, creator_code: String) -> Result<Message, String> {
    let api_key = get_public_api_key();

    match api_key {
        Ok (api_key) => {
            let client = reqwest::Client::new();
            let res = client.post(format!("{0}/accounts/{1}/baskets/{2}/creator-codes", BASE_URL, api_key, basket_identifier))
                .body(json!({
                    "creator_code": creator_code
                }).to_string())
                .send()
                .await;

            match res {
                Ok (data) => {
                    let json = data.json::<Message>()
                        .await;

                    match json {
                        Ok (json_data) => {
                            return Ok(json_data);
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

/// Remove the current creator code for basket with specific id
pub async fn remove_creator_code(basket_identifier: String) -> Result<Message, String> {
    let api_key = get_public_api_key();

    match api_key {
        Ok (api_key) => {
            let client = reqwest::Client::new();
            let res = client.post(format!("{0}/accounts/{1}/baskets/{2}/creator-codes/remove", BASE_URL, api_key, basket_identifier))
                .send()
                .await;

            match res {
                Ok (data) => {
                    let json = data.json::<Message>()
                        .await;

                    match json {
                        Ok (json_data) => {
                            return Ok(json_data);
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