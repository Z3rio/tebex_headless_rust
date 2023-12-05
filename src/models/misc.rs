use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Data<T> {
    pub data: T
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct FieldOfDetail {
    
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Meta {
    
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Message {
    pub status: Option<i32>,
    pub r#type: Option<String>,
    pub title: Option<String>,
    pub detail: Option<String>,
    pub error_code: Option<String>,
    pub field_of_details: Option<Vec<FieldOfDetail>>,
    pub meta: Option<Vec<Meta>>,
    pub message: Option<String>
}