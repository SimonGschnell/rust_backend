use std::fmt::Debug;

use actix_web::{
    body::BoxBody, get, http::header::ContentType, web, Either, Error, HttpResponse, Responder,
    ResponseError,
};
pub mod models;
use serde::Serialize;

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

//? HttpResponse with custom type
//* add the derive trait Serialize to the custom type
#[derive(Serialize, Debug)]
struct Custom {
    address: String,
    valid: bool,
}

//* implement the Responder trait for the custom type
impl Responder for Custom {
    type Body = BoxBody;
    fn respond_to(self, _: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}

//* now we are able to return the custom type freely
pub async fn return_custom_type() -> impl Responder {
    Custom {
        address: String::from("192.168.0.1"),
        valid: true,
    }
}

//? handler with that can return different HttpResponses

type EitherResult = Either<HttpResponse, Result<String, actix_web::Error>>;
async fn multiple_responses(data: web::Data<bool>) -> EitherResult {
    let data = data.into_inner();
    if *data {
        Either::Left(HttpResponse::BadRequest().body("Bad request"))
    } else {
        Either::Right(Ok(String::from("OK")))
    }
}

//? Error handling
#[derive(Debug)]
pub struct CustomError(String);

impl std::fmt::Display for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for CustomError {}
impl ResponseError for CustomError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        actix_web::http::StatusCode::FORBIDDEN
    }
    fn error_response(&self) -> HttpResponse<BoxBody> {
        HttpResponse::build(self.status_code())
            .content_type("text/plain")
            .body(String::from(&self.0) + " => is the message of the error that i customly created")
    }
}

#[get("/error/{bool}")]
pub async fn error_respose(data: web::Path<bool>) -> Result<impl Responder, CustomError> {
    let data = data.into_inner();
    if data {
        Ok("OK")
    } else {
        Err(CustomError(String::from("custom error")))
    }
}
