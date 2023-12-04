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
    pub image: String,
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