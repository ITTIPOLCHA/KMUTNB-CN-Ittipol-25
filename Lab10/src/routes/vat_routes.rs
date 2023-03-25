use actix_web::web;
use crate::handlers::vat_handler::*;


pub fn config( cfg: &mut web::ServiceConfig ){

    cfg
        .service( get_vat )
        .service( post_vat );

}