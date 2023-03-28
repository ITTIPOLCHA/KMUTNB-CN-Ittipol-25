use actix_web::{ web, get, Responder, HttpResponse, patch, delete};
use log::{debug, info};

use crate::models::feature::{TaxMenu, UpdateRequest, TaxResponse};


#[get("/feature/admin/{user_id}")]
async fn get_feature(_user_id: web::Path<i32>) -> impl Responder {
    info!("get feature");
    let url = format!("https://private-f5d89-ittipolcha.apiary-mock.com/feature/{}", _user_id);

    let response = reqwest::get(&url)
        .await
        .unwrap()
        .json::<TaxMenu>()
        .await
        .unwrap();

    let _user_id = _user_id.into_inner() as i32;
    let mut _r = "client";
    if _user_id == 0 {
      _r = "admin";
    }
    
    if response.right == _r.to_string() {
        debug!("patch: ✅");
        HttpResponse::Ok().json(response)
    }
    else {
        debug!("patch: ❌");
        HttpResponse::NotFound().body("No access")
    }

}

#[patch("/feature/admin/{user_id}")]
async fn patch_feature(_user_id: web::Path<i32>, data: web::Json<UpdateRequest>) -> impl Responder {
    info!("patch feature");

    let _data = data.into_inner();

    let _user_id = _user_id.into_inner() as i32;
    let mut _r = "client";
    if _user_id == 0 {
      _r = "admin";
    }

    if _data.right == _r.to_string() {
        debug!("patch: ✅");
        HttpResponse::Ok().body("Update complete")
    }
    else {
        debug!("patch: ❌");
        HttpResponse::NotFound().body("No access")
    }
}

#[delete("/feature/admin/{user_id}")]
async fn delete_feature(_user_id: web::Path<i32>) -> impl Responder {
    info!("delete feature");

    let _user_id = _user_id.into_inner() as i32;

    let _t = "VAT";
    let _ra = "2015-08-05T08:40:51.620Z";

    let res_feature = TaxResponse {
        user_id: 0,
        tax: _t.to_string(),
        response_at: _ra.to_string(),
     };

    if res_feature.user_id == _user_id {
        debug!("delete: ✅");
        HttpResponse::Ok().body("Delete complete")
    }
    else {
        debug!("delete: ❌");
        HttpResponse::NotFound().body("No access")
    }
}