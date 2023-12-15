#[cfg(test)]
mod tests {
    use crate::handlers;

    #[tokio::test]
    async fn try_fetch_categories() {
        handlers::category::get_all_categories(false, None, None).await.expect("Categories could not be fetched.");
    }

    #[tokio::test]
    async fn try_fetch_category() -> Result<(), String> {
        let categories = handlers::category::get_all_categories(false, None, None).await.expect("Packages could not be fetched.");

        if categories.len() < 1 {
            return Err(String::from("Cant test this, the length of packages has to atleast be 1"));
        }

        handlers::category::get_category(categories[0].id, true, None, None).await.expect("Package could not be fetched");

        return Ok(())
    }
}