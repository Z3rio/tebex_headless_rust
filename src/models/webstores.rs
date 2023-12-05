use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
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