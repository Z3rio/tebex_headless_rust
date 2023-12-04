#[cfg(test)]
mod tests {
    use crate::{handlers::{basket::{create_basket, get_basket, get_basket_auth_url, add_package_to_basket, remove_package_from_basket, update_package_basket_quantity}, self, webstores::get_webstore, coupons::{apply_coupon, remove_coupon}, package::get_all_packages}, get_env_var};

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
                return Err(String::from(format!("Error trying to get basket, {0}", err)));
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
                return Err(String::from(format!("Error trying to get basket auth url, {0}", err)));
            }
        }
    }

    // basketes (package handling)
    #[tokio::test]
    async fn try_add_package_to_basket() -> Result<(), String> {
        let basket = create_basket(get_local_ip(), None).await;

        match basket {
            Ok(basket) => {
                let packages = get_all_packages().await;

                match packages {
                    Ok(packages) => {
                        if packages.len() < 1 {
                            return Err(String::from("Cant test this, the length of packages has to atleast be 1"));
                        }

                        add_package_to_basket(basket.ident, packages[0].id, 1, packages[0].r#type.clone()).await.expect("Could not add package to basket");
                        return Ok(());
                    }
        
                    Err (err) => {
                        return Err(String::from(format!("Error trying to get basket, {0}", err)));
                    }
                }
            }

            Err (err) => {
                return Err(String::from(format!("Error trying to get basket, {0}", err)));
            }
        }
    }

    #[tokio::test]
    async fn try_remove_package_from_basket() -> Result<(), String> {
        let basket = create_basket(get_local_ip(), None).await;

        match basket {
            Ok(basket) => {
                let packages = get_all_packages().await;

                match packages {
                    Ok(packages) => {
                        if packages.len() < 1 {
                            return Err(String::from("Cant test this, the length of packages has to atleast be 1"));
                        }

                        add_package_to_basket(basket.ident.clone(), packages[0].id, 1, packages[0].r#type.clone()).await.expect("Could not add package to basket");
                        remove_package_from_basket(basket.ident.clone(), packages[0].id).await.expect("Could not remove package to basket");
                        return Ok(());
                    }
        
                    Err (err) => {
                        return Err(String::from(format!("Error trying to get basket, {0}", err)));
                    }
                }
            }

            Err (err) => {
                return Err(String::from(format!("Error trying to get basket, {0}", err)));
            }
        }
    }

    #[tokio::test]
    async fn try_update_package_basket_quantity() -> Result<(), String> {
        let basket = create_basket(get_local_ip(), None).await;

        match basket {
            Ok(basket) => {
                let packages = get_all_packages().await;

                match packages {
                    Ok(packages) => {
                        if packages.len() < 1 {
                            return Err(String::from("Cant test this, the length of packages has to atleast be 1"));
                        }

                        add_package_to_basket(basket.ident.clone(), packages[0].id, 1, packages[0].r#type.clone()).await.expect("Could not add package to basket");
                        update_package_basket_quantity(basket.ident.clone(), packages[0].id, 2).await.expect("Could not update package quantity");
                        return Ok(());
                    }
        
                    Err (err) => {
                        return Err(String::from(format!("Error trying to get basket, {0}", err)));
                    }
                }
            }

            Err (err) => {
                return Err(String::from(format!("Error trying to get basket, {0}", err)));
            }
        }
    }

    // webstores
    #[tokio::test]
    async fn try_get_webstore() -> Result<(), String> {
        get_webstore().await.expect("Could not get webstore");

        return Ok(());
    }

    // coupons
    #[tokio::test]
    async fn try_apply_coupon() -> Result<(), String> {
        let basket = create_basket(get_local_ip(), None).await;

        match basket {
            Ok(basket) => {
                let applied = apply_coupon(basket.ident.clone(), String::from("coupon_code")).await;

                match applied {
                    Ok(_) => {
                        return Ok(())
                    }

                    Err (err) => {
                        return Err(String::from(format!("Error trying to apply coupon, {0}", err)));
                    }
                }

            }

            Err (err) => {
                return Err(String::from(format!("Error trying to create basket, {0}", err)));
            }
        }
    }

    #[tokio::test]
    async fn try_remove_coupon() -> Result<(), String> {
        let basket = create_basket(get_local_ip(), None).await;

        match basket {
            Ok(basket) => {
                let applied = apply_coupon(basket.ident.clone(), String::from("coupon_code")).await;

                match applied {
                    Ok(_) => {
                        let removed = remove_coupon(basket.ident.clone(), String::from("coupon_code")).await;

                        match removed {
                            Ok(_) => {
                                return Ok(())
                            }
        
                            Err (err) => {
                                return Err(String::from(format!("Error trying to remove coupon, {0}", err)));
                            }
                        }
                    }

                    Err (err) => {
                        return Err(String::from(format!("Error trying to apply coupon, {0}", err)));
                    }
                }

            }

            Err (err) => {
                return Err(String::from(format!("Error trying to create basket, {0}", err)));
            }
        }
    }
}
