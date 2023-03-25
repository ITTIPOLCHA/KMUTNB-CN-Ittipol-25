use actix_web::web;
use crate::handlers::feature_handler::*;


pub fn config( cfg: &mut web::ServiceConfig ){

    cfg
        .service( get_feature )
        .service( patch_feature )
        .service( delete_feature );

}