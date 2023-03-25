use actix_web::{get, Responder, HttpResponse};
use log::{debug, info};

use crate::models::tax::Tax;


#[get("/tax")]
async fn get_tax() -> impl Responder {
    info!("get tax");
    debug!("get: ✅");

    let url = "https://private-f5d89-ittipolcha.apiary-mock.com/tax";

    let response = reqwest::get(url)
        .await
        .unwrap()
        .json::<Tax>()
        .await.unwrap();

    HttpResponse::Ok().json(response)
}