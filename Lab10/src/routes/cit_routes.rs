use actix_web::web;
use crate::handlers::cit_handler::{get_cit, post_cit} ;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(get_cit)
        .service(post_cit);
}