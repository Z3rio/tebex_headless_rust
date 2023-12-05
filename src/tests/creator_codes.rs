#[cfg(test)]
mod tests {
    use crate::handlers::{basket::create_basket, creator_codes::{apply_creator_code, remove_creator_code}};

    #[tokio::test]
    async fn try_apply_creator_code() -> Result<(), String> {
        let basket = create_basket(local_ip_address::local_ip().unwrap().to_string(), None).await;

        match basket {
            Ok(basket) => {
                let applied = apply_creator_code(basket.ident.clone(), String::from("creator_code")).await;

                match applied {
                    Ok(_) => {
                        return Ok(())
                    }

                    Err (err) => {
                        return Err(String::from(format!("Error trying to apply creator code, {0}", err)));
                    }
                }

            }

            Err (err) => {
                return Err(String::from(format!("Error trying to create basket, {0}", err)));
            }
        }
    }

    #[tokio::test]
    async fn try_remove_creator_code() -> Result<(), String> {
        let basket = create_basket(local_ip_address::local_ip().unwrap().to_string(), None).await;

        match basket {
            Ok(basket) => {
                let applied = apply_creator_code(basket.ident.clone(), String::from("creator_code")).await;

                match applied {
                    Ok(_) => {
                        let removed = remove_creator_code(basket.ident.clone()).await;

                        match removed {
                            Ok(_) => {
                                return Ok(())
                            }
        
                            Err (err) => {
                                return Err(String::from(format!("Error trying to remove creator code, {0}", err)));
                            }
                        }
                    }

                    Err (err) => {
                        return Err(String::from(format!("Error trying to apply creator code, {0}", err)));
                    }
                }

            }

            Err (err) => {
                return Err(String::from(format!("Error trying to create basket, {0}", err)));
            }
        }
    }
}