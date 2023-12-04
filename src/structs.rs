use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Category {
    pub id: i32,
    pub name: String
}

#[derive(Deserialize, Serialize)]
pub struct Package {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub image: Option<String>,
    pub r#type: String,
    pub category: Category,
    pub base_price: f32,
    pub sales_tax: f32,
    pub total_price: f32,
    pub discount: f32,
    pub disable_quantity: bool,
    pub disable_gifting: bool,
    pub expiration_date: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>
}

#[derive(Deserialize, Serialize)]
pub struct Data<T> {
    pub data: T
}

#[derive(Deserialize, Serialize)]
pub struct CategoryParent {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub packages: Vec<Package>,
    pub order: i32,
    pub display_type: String
}

#[derive(Deserialize, Serialize)]
pub struct FullCategory {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub parent: Option<CategoryParent>,
    pub order: i32,
    pub display_type: String
}

#[derive(Deserialize, Serialize)]
pub struct Giftcard {

}

#[derive(Deserialize, Serialize)]
pub struct Coupon {
    
}

#[derive(Deserialize, Serialize)]
pub struct Basket {
    pub ident: String,
    pub complete: bool,
    pub id: i32,
    pub country: String,
    pub ip: String,
    pub username_id: Option<i32>,
    pub username: Option<String>,
    pub base_price: f32,
    pub sales_tax: f32,
    pub total_price: f32,
    pub packages: Vec<Package>,
    pub coupons: Vec<Coupon>,
    pub giftcards: Vec<Giftcard>,
    pub creator_code: Option<String>,
    pub links: Vec<String>    
}

#[derive(Deserialize, Serialize)]
pub struct BasketUrl {
    pub name: String,
    pub url: String
}

#[derive(Deserialize, Serialize)]
pub struct Webstore {
    pub id: i32,
    pub description: String,
    pub name: String,
    pub webstore_url: String,
    pub currency: String,
    pub lang: String,
    pub logo: Option<String>,
    pub platform_type: String,
    pub created_at: Option<String>
}