use actix_web::{get, web, Responder};
pub mod models;
use serde::Deserialize;

pub async fn index_counter(data: web::Data<models::Counter>) -> impl Responder {
    let mut counter = data.value.lock().unwrap();
    *counter += 1;

    format!("the new value of the counter is :{}", counter)
}

pub async fn index_root(data: web::Data<models::WebAppName>) -> impl Responder {
    format!("the name of the app is :{}", data.name)
}

#[get("/users/{user_id}/{user_name}")]
pub async fn path_route(path: web::Path<(u32, String)>) -> impl Responder {
    let (user_id, user_name) = path.into_inner();
    format!(
        "user :{} with id: {} has accessed this website",
        user_name, user_id
    )
}

#[get("/posts/{post_id}/{post_name}")]
pub async fn post_route(post: web::Path<models::Post>) -> impl Responder {
    format!(
        "received: {:?} | post: {} with id: {} was posted",
        post, post.post_name, post.post_id
    )
}

//web::Json extractor is used for POST requests and serde::Deserialize is needed on the linked model
pub async fn post_req(data: web::Json<models::PostRequest>) -> impl Responder {
    format!(
        "the following car was send to the server: {}-{}-with price:{} and extras: {:?}\n{:?}",
        data.color, data.brand, data.price, data.extras, data
    )
}

//web::Query extractor is used for QUERY parameters in the request
pub async fn query_req(data: web::Query<models::PostRequest>) -> impl Responder {
    format!("the following car was send: {:?}", data)
}
