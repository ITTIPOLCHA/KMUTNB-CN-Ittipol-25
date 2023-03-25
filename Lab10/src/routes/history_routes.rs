use actix_web::web;
use crate::handlers::history_handler::*;


pub fn config( cfg: &mut web::ServiceConfig ){

    cfg
        .service(get_history);

}