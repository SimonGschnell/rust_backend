#![allow(clippy::redundant_clone)]
use super::handlers;
use super::handlers::models;

use actix_web::web;
use std::sync::Mutex;

pub fn counter_scope(cfg: &mut web::ServiceConfig) {
    let counter = web::Data::new(models::Counter {
        value: Mutex::new(0),
    });
    //? we have to move the counter inside because
    //? otherwise it won't be able to share its state with other threads

    cfg.app_data(counter.clone())
        .route("", web::get().to(handlers::index_counter));

    cfg.app_data(counter.clone())
        .route("other_counter", web::get().to(handlers::index_counter));
}

pub fn test_scope(cfg: &mut web::ServiceConfig) {
    cfg.app_data(web::Data::new(models::WebAppName {
        name: String::from("simons_app"),
    }))
    .route("", web::get().to(handlers::index_root));
}

pub fn car_scope(cfg: &mut web::ServiceConfig) {
    //? adds a configuration on the payload of the post request (maximal 4kb payload limit)
    let json_config = web::JsonConfig::default().limit(4096);
    //? to apply the json_config we only need to pass it as app_data
    cfg.app_data(json_config)
        .route("post", web::post().to(handlers::post_req));
    cfg.route("query", web::get().to(handlers::query_req));
}
