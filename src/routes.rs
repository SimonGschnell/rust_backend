use actix_web::{guard, web, HttpResponse};
mod handlers;
mod scopes;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(handlers::path_route)
        .service(handlers::post_route);
    cfg.service(web::scope("/test").configure(scopes::test_scope));
    cfg.service(web::scope("/counter").configure(scopes::counter_scope));
    cfg.service(web::scope("/car").configure(scopes::car_scope));
}
