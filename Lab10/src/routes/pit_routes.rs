use actix_web::web;
use crate::handlers::pit_handler::{get_pit} ;


pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(get_pit);
}