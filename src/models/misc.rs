use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct Data<T> {
    pub data: T
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Message {
    pub message: String
}