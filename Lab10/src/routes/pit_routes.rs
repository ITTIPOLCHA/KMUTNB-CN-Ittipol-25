use actix_web::web;
use crate::handlers::pit_handler::*;


pub fn config( cfg: &mut web::ServiceConfig ){

    cfg
        .service( get_pit )
        .service( post_pit );

}