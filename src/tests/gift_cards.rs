#[cfg(test)]
mod tests {
    use crate::handlers::{basket::create_basket, gift_cards::{apply_gift_card, remove_gift_card}};

    #[tokio::test]
    async fn try_apply_gift_card() -> Result<(), String> {
        let basket = create_basket(local_ip_address::local_ip().unwrap().to_string(), None).await;

        match basket {
            Ok(basket) => {
                let applied = apply_gift_card(basket.ident.clone(), String::from("gift_card")).await;

                match applied {
                    Ok(_) => {
                        return Ok(())
                    }

                    Err (err) => {
                        return Err(String::from(format!("Error trying to apply giftcard, {0}", err)));
                    }
                }

            }

            Err (err) => {
                return Err(String::from(format!("Error trying to create basket, {0}", err)));
            }
        }
    }

    #[tokio::test]
    async fn try_remove_gift_card() -> Result<(), String> {
        let basket = create_basket(local_ip_address::local_ip().unwrap().to_string(), None).await;

        match basket {
            Ok(basket) => {
                let applied = apply_gift_card(basket.ident.clone(), String::from("gift_card")).await;

                match applied {
                    Ok(_) => {
                        let removed = remove_gift_card(basket.ident.clone(), String::from("gift_card")).await;

                        match removed {
                            Ok(_) => {
                                return Ok(())
                            }
        
                            Err (err) => {
                                return Err(String::from(format!("Error trying to remove giftcard, {0}", err)));
                            }
                        }
                    }

                    Err (err) => {
                        return Err(String::from(format!("Error trying to apply giftcard, {0}", err)));
                    }
                }

            }

            Err (err) => {
                return Err(String::from(format!("Error trying to create basket, {0}", err)));
            }
        }
    }
}