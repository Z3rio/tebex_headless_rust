use serde::{Deserialize, Serialize};
use super::{packages::Package, coupons::Coupon, gift_cards::Giftcard};

#[derive(Deserialize, Serialize, Clone)]
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

#[derive(Deserialize, Serialize, Clone)]
pub struct BasketUrl {
    pub name: String,
    pub url: String
}