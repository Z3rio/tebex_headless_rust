#[cfg(test)]
mod tests {
    use crate::handlers::webstores::get_webstore;

    #[tokio::test]
    async fn try_get_webstore() -> Result<(), String> {
        get_webstore().await.expect("Could not get webstore");

        return Ok(());
    }
}