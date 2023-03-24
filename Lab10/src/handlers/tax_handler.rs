use actix_web::{get, Responder, HttpResponse};

use crate::models::tax::Tax;


#[get("/tax")]
async fn get_tax() -> impl Responder {
    let url = "https://private-f5d89-ittipolcha.apiary-mock.com/tax";
    let response = reqwest::get(url).await.unwrap().json::<Vec<Tax>>().await.unwrap();
    HttpResponse::Ok().json(response)
}