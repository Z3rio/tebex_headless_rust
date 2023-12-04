use serde::{Deserialize, Serialize};
use super::packages::Package;

#[derive(Deserialize, Serialize, Clone)]
pub struct Category {
    pub id: i32,
    pub name: String
}

#[derive(Deserialize, Serialize, Clone)]
pub struct CategoryParent {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub packages: Vec<Package>,
    pub order: i32,
    pub display_type: String
}

#[derive(Deserialize, Serialize, Clone)]
pub struct FullCategory {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub parent: Option<CategoryParent>,
    pub order: i32,
    pub display_type: String
}