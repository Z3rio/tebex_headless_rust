//! This module contains the functions for adding/removing gift cards

use serde_json::json;

use crate::{BASE_URL, models::misc::Message};

use super::misc::get_public_api_key;

/// Apply gift card to basket with specific id
pub async fn apply_gift_card(basket_identifier: String, card_number: String) -> Result<Message, String> {
    let api_key = get_public_api_key();

    match api_key {
        Ok (api_key) => {
            let client = reqwest::Client::new();
            let res = client.post(format!("{0}/accounts/{1}/baskets/{2}/giftcards", BASE_URL, api_key, basket_identifier))
                .body(json!({
                    "card_number": card_number
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

/// Remove specific gift card from basket with specific id
pub async fn remove_gift_card(basket_identifier: String, card_number: String) -> Result<Message, String> {
    let api_key = get_public_api_key();

    match api_key {
        Ok (api_key) => {
            let client = reqwest::Client::new();
            let res = client.post(format!("{0}/accounts/{1}/baskets/{2}/giftcards/remove", BASE_URL, api_key, basket_identifier))
                .body(json!({
                    "card_number": card_number
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