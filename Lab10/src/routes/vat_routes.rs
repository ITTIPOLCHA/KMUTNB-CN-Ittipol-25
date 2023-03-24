use actix_web::web;
use crate::handlers::vat_handler::{get_vat, post_vat} ;


pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(get_vat)
        .service(post_vat);
}