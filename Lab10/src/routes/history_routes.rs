use actix_web::web;
use crate::handlers::history_handler::{get_history} ;


pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(get_history);
}