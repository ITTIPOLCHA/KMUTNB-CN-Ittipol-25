use actix_web::{get, post, web, Responder, HttpResponse};
use serde::{Deserialize, Serialize};
use log::{debug, info};
use serde_json::json;

use crate::models::vat::Vat;

#[derive(Debug, Serialize, Deserialize)]
struct DATA {
  types: String,
  product_name: String,
  product_price: i32,
  service_name: String,
  service_price: i32,
}

#[derive(Debug, Serialize, Deserialize)]
struct InputVat {
  user_id: i32,
  data: DATA,
}

#[get("/tax/vat")]
async fn get_vat() -> impl Responder {
    let url = "https://private-f5d89-ittipolcha.apiary-mock.com/tax/vat";
    let response = reqwest::get(url).await.unwrap().json::<Vec<Vat>>().await.unwrap();
    HttpResponse::Ok().json(response)
}

#[post("/tax/vat")]
async fn post_vat(data: web::Json<InputVat>) -> impl Responder {
    info!("post vat");
    debug!("post: ✅");

    let _data = data.into_inner();
    
    let mut _ni = 0;
    let mut _price = 0;
    let _vat = "7%";

    if _data.data.types == "Product" {
        _ni = _data.data.product_price* 7/ 100;
        _price = _data.data.product_price+ _ni;
    }
    else if _data.data.types == "Service" {
        _ni = _data.data.service_price* 7/ 100;
        _price = _data.data.service_price+ _ni;
    }

    let _ra = "2015-08-05T08:40:51.620Z";

    #[derive(Serialize, Deserialize)]
    struct ResVat {
        vat: String,
        net_income: i32,
        price: i32,
        response_at: String,
    }

    let res_vat = ResVat {
        vat: _vat.to_string(),
        net_income: _ni,
        price: _price,
        response_at: _ra.to_string(),
    };
     
    let response_body = json!(res_vat);

    HttpResponse::Created().json(response_body)
}