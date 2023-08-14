use serde::Deserialize;
use std::sync::Mutex;

pub struct WebAppName {
    pub name: String,
}

pub struct Counter {
    pub value: Mutex<i32>,
}

#[derive(Deserialize, Debug)]
pub struct Post {
    pub post_id: i32,
    pub post_name: String,
}

#[derive(Deserialize, Debug)]
pub struct PostRequest {
    pub color: String,
    pub brand: String,
    pub price: i32,
    pub extras: Option<bool>,
}
