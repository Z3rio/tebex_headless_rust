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
    use super::*;

    #[test]
    fn check_env_vars() {
        let public_key = get_env_var("TEBEX_PUBLIC_KEY");

        public_key.expect("TEBEX_PUBLIC_KEY is not present in your .env file");
    }

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
}
