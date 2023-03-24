use actix_web::{web, get, Responder, HttpResponse};

use crate::models::history::{User};


#[get("/history")]
async fn get_history() -> impl Responder {
    let url = "https://private-f5d89-ittipolcha.apiary-mock.com/history/user_id";
    let response = reqwest::get(url).await.unwrap().json::<Vec<User>>().await.unwrap();

    HttpResponse::Ok().json(response)
}