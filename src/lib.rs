pub mod structs;
pub mod handlers;

use std::env::VarError;
use dotenv::dotenv;

pub static BASE_URL: &str = "https://headless.tebex.io/api";

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

fn get_env_var(query: &str) -> Result<String, VarError> {
    if cfg!(test) && !dotenv().is_ok() {
        dotenv().ok();
    }

    let queried: Result<String, VarError> = std::env::var(query);

    return queried;
}

#[cfg(test)]
mod tests {
    use crate::handlers::basket::{create_basket, get_basket, get_basket_auth_url};

    use super::*;

    fn get_local_ip() -> String {
        return local_ip_address::local_ip().unwrap().to_string();
    }

    // misc
    #[test]
    fn check_env_vars() {
        let public_key = get_env_var("TEBEX_PUBLIC_KEY");

        public_key.expect("TEBEX_PUBLIC_KEY is not present in your .env file");
    }

    // packages
    #[tokio::test]
    async fn try_fetch_packages() {
        handlers::package::get_all_packages().await.expect("Packages could not be fetched.");
    }

    #[tokio::test]
    async fn try_fetch_package() -> Result<(), String> {
        let packages = handlers::package::get_all_packages().await.expect("Packages could not be fetched.");

        if packages.len() < 1 {
            return Err(String::from("Cant test this, the length of packages has to atleast be 1"));
        }

        handlers::package::get_package(packages[0].id).await.expect("Package could not be fetched");

        return Ok(())
    }

    // categories
    #[tokio::test]
    async fn try_fetch_categories() {
        handlers::category::get_all_categories(false).await.expect("Categories could not be fetched.");
    }

    #[tokio::test]
    async fn try_fetch_category() -> Result<(), String> {
        let categories = handlers::category::get_all_categories(false).await.expect("Packages could not be fetched.");

        if categories.len() < 1 {
            return Err(String::from("Cant test this, the length of packages has to atleast be 1"));
        }

        handlers::category::get_category(categories[0].id, true).await.expect("Package could not be fetched");

        return Ok(())
    }

    // baskets
    #[tokio::test]
    async fn try_create_basket() -> Result<(), String> {
        create_basket(get_local_ip(), None).await.expect("Could not create basket");

        return Ok(())
    }

    #[tokio::test]
    async fn try_get_basket() -> Result<(), String> {
        let basket = create_basket(get_local_ip(), None).await;

        match basket {
            Ok(basket) => {
                get_basket(basket.ident).await.expect("Could not get created basket");

                return Ok(())
            }

            Err (err) => {
                return Err(String::from(format!("Error trying to create basket, {0}", err)));
            }
        }
    }

    #[tokio::test]
    async fn try_get_basket_auth_url() -> Result<(), String> {
        let basket = create_basket(get_local_ip(), None).await;

        match basket {
            Ok(basket) => {
                let basket_url = get_basket_auth_url(basket.ident, String::from("https://www.example.com")).await.expect("Could not get basket url");

                if basket_url.len() < 1 {
                    return Err(String::from("The length of returned basket urls were 0, they should atleast be 1, aka not empty"));
                }

                return Ok(())
            }

            Err (err) => {
                return Err(String::from(format!("Error trying to create basket, {0}", err)));
            }
        }
    }
}
