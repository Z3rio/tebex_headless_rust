mod structs;

use crate::structs::{Package, Data};

use std::env::VarError;
use dotenv::dotenv;

static BASE_URL: &str = "https://headless.tebex.io/api";

pub fn set_public_api_key(api_key: String) {
    std::env::set_var("PUBLIC_API_KEY", api_key);
}

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

pub async fn get_package(package_id: i32) -> Result<Package, String> {
    let api_key = get_public_api_key();

    match api_key {
        Ok (api_key) => {
            let res = reqwest::get(format!("{0}/accounts/{1}/packages/{2}", BASE_URL, api_key, package_id))
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

pub async fn get_all_packages() -> Result<Vec<Package>, String> {
    let api_key = get_public_api_key();

    match api_key {
        Ok (api_key) => {
            let res = reqwest::get(format!("{0}/accounts/{1}/packages", BASE_URL, api_key))
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

fn get_env_var(query: &str) -> Result<String, VarError> {
    if cfg!(test) && !dotenv().is_ok() {
        dotenv().ok();
    }

    let queried: Result<String, VarError> = std::env::var(query);

    return queried;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_env_vars() {
        let public_key = get_env_var("TEBEX_PUBLIC_KEY");

        public_key.expect("TEBEX_PUBLIC_KEY is not present in your .env file");
    }

    #[tokio::test]
    async fn try_fetch_packages() {
        get_all_packages().await.expect("Packages could not be fetched.");
    }

    #[tokio::test]
    async fn try_fetch_package() -> Result<(), String> {
        let packages = get_all_packages().await.expect("Packages could not be fetched.");

        if packages.len() < 1 {
            return Err(String::from("Cant test this, the length of packages has to atleast be 1"));
        }

        get_package(packages[0].id).await.expect("Package could not be fetched");

        return Ok(())
    }
}
