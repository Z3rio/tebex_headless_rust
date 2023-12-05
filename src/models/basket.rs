use serde::{Deserialize, Serialize};
use super::{packages::Package, coupons::Coupon, gift_cards::Giftcard, misc::{FieldOfDetail, Meta}};

#[derive(Deserialize, Serialize, Clone, Debug)]
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

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct BasketUrl {
    pub name: String,
    pub url: String
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct PackageRetVal {
    pub data: Option<Basket>,

    // from ./models/misc.rs
    pub status: Option<i32>,
    pub r#type: Option<String>,
    pub title: Option<String>,
    pub detail: Option<String>,
    pub error_code: Option<String>,
    pub field_of_details: Option<Vec<FieldOfDetail>>,
    pub meta: Option<Vec<Meta>>,
    pub message: Option<String>
}