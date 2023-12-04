use serde_json::json;

use crate::{structs::{Basket, Data, BasketUrl}, get_public_api_key, BASE_URL};

pub async fn get_basket(basket_identifier: String) -> Result<Basket, String> {
    let api_key = get_public_api_key();

    match api_key {
        Ok (api_key) => {
            let res = reqwest::get(format!("{0}/accounts/{1}/baskets/{2}", BASE_URL, api_key, basket_identifier))
                .await;

            match res {
                Ok (data) => {
                    let json = data.json::<Data<Basket>>()
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

pub async fn get_basket_auth_url(basket_identifier: String, return_url: String) -> Result<Vec<BasketUrl>, String> {
    let api_key = get_public_api_key();

    match api_key {
        Ok (api_key) => {
            let res = reqwest::get(format!("{0}/accounts/{1}/baskets/{2}/auth?returnUrl={3}", BASE_URL, api_key, basket_identifier, return_url))
                .await;

            match res {
                Ok (data) => {
                    let json = data.json::<Vec<BasketUrl>>()
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

pub async fn create_basket(ip_address: String, username: Option<String>) -> Result<Basket, String> {
    let api_key = get_public_api_key();

    match api_key {
        Ok (api_key) => {
            let client = reqwest::Client::new();
            let res = client.post(format!("{0}/accounts/{1}/baskets", BASE_URL, api_key))
                .body((match username {
                    None => json!({
                        "ip_address": ip_address
                    }),
                    
                    Some(username) => json!({
                        "ip_address": ip_address,
                        "username": username
                    })
                }).to_string())
                .send()
                .await;

            match res {
                Ok (data) => {
                    let json = data.json::<Data<Basket>>()
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

pub async fn add_package_to_basket(basket_identifier: String, package_id: i32, quantity: i32, r#type: String) -> Result<Basket, String> {
    if r#type != "single" && r#type != "subscription" {
        return Err(String::from("The package type has to either be \"single\" or \"subscription\"!"))
    }
    let client = reqwest::Client::new();

    let res = client.post(format!("{0}/baskets/{1}/packages", BASE_URL, basket_identifier))
        .body(json!({
            "package_id": package_id,
            "quantity": quantity,
            "type": r#type
        }).to_string())
        .send()
        .await;

    match res {
        Ok (data) => {
            let json = data.json::<Data<Basket>>()
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

pub async fn remove_package_from_basket(basket_identifier: String, package_id: i32) -> Result<Basket, String> {
    let client = reqwest::Client::new();
    let res = client.post(format!("{0}/baskets/{1}/packages/remove", BASE_URL, basket_identifier))
        .body(json!({
            "package_id": package_id,
        }).to_string())
        .send()
        .await;

    match res {
        Ok (data) => {
            let json = data.json::<Data<Basket>>()
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

pub async fn update_package_basket_quantity(basket_identifier: String, package_id: i32, quantity: i32) -> Result<Basket, String> {
    let client = reqwest::Client::new();
    let res = client.put(format!("{0}/baskets/{1}/packages/{2}", BASE_URL, basket_identifier, package_id))
        .body(json!({
            "quantity": quantity
        }).to_string())
        .send()
        .await;

    match res {
        Ok (data) => {
            let json = data.json::<Data<Basket>>()
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

pub async fn add_gift_package_to_basket(basket_identifier: String, package_id: i32, target_username: String) -> Result<Basket, String> {
    let client = reqwest::Client::new();
    let res = client.post(format!("{0}/baskets/{1}/packages", BASE_URL, basket_identifier))
        .body(json!({
            "package_id": package_id,
            "target_username_id": target_username
        }).to_string())
        .send()
        .await;

    match res {
        Ok (data) => {
            let json = data.json::<Data<Basket>>()
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