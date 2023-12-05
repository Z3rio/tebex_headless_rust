#[cfg(test)]
pub mod tests {
    use crate::handlers::misc::get_env_var;

    #[test]
    fn check_env_vars() {
        let public_key = get_env_var("TEBEX_PUBLIC_KEY");

        public_key.expect("TEBEX_PUBLIC_KEY is not present in your .env file");
    }
}

pub fn get_local_ip() -> String {
    return local_ip_address::local_ip().unwrap().to_string();
}