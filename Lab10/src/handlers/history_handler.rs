use actix_web::{ web, get, Responder, HttpResponse};
use log::{debug, info};

use crate::models::history::History;


#[get("/history/{user_id}")]
async fn get_history(_user_id: web::Path<i32>) -> impl Responder {
    info!("get history");

    let url = format!("https://private-f5d89-ittipolcha.apiary-mock.com/history/{}", _user_id);

    let response = reqwest::get(&url)
        .await
        .unwrap()
        .json::<History>()
        .await
        .unwrap();

    let _user_id = _user_id.into_inner() as i32;
    
    if response.user_id == _user_id {
        debug!("get: ✅");
        HttpResponse::Ok().json(response)
    }
    else {
        debug!("get: ❌");
        HttpResponse::NotFound().body("User ID not found")
    }

}