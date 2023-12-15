#[cfg(test)]
mod tests {
    use crate::handlers;

    #[tokio::test]
    async fn try_fetch_packages() {
        handlers::package::get_all_packages(None, None).await.expect("Packages could not be fetched.");
    }

    #[tokio::test]
    async fn try_fetch_package() -> Result<(), String> {
        let packages = handlers::package::get_all_packages(None, None).await.expect("Packages could not be fetched.");

        if packages.len() < 1 {
            return Err(String::from("Cant test this, the length of packages has to atleast be 1"));
        }

        handlers::package::get_package(packages[0].id, None, None).await.expect("Package could not be fetched");

        return Ok(())
    }
}