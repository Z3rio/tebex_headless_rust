#[cfg(test)]
mod tests {
    use crate::{handlers::{coupons::{apply_coupon, remove_coupon}, basket::create_basket}, tests::misc::get_local_ip};

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