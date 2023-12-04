mod structs;

use std::env::VarError;

use dotenv::dotenv;

pub fn get_env_var(query: &str) -> Result<String, VarError> {
    if !dotenv().is_ok() {
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
}
