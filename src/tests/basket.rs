#[cfg(test)]
mod tests {
    use crate::{handlers::{basket::{create_basket, get_basket, get_basket_auth_url, add_package_to_basket, remove_package_from_basket, update_package_basket_quantity}, package::get_all_packages}, tests::misc::get_local_ip};

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
}