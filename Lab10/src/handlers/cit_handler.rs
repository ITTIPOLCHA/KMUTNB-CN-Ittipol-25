use actix_web::{get, post, web, Responder, HttpResponse};
use log::{debug, info};
use serde_json::json;

use crate::models::cit::{Cit, InputCit, ResCit};

#[get("/tax/cit")]
async fn get_cit() -> impl Responder {
    info!("get cit");
    debug!("get: ✅");

    let url = "https://private-f5d89-ittipolcha.apiary-mock.com/tax/cit";

    let response = reqwest::get(url)
      .await
      .unwrap()
      .json::<Cit>()
      .await
      .unwrap();

    HttpResponse::Ok().json(response)
}

#[post("/tax/cit")]
async fn post_cit(data: web::Json<InputCit>) -> impl Responder {
    info!("post cit");
    debug!("post: ✅");

    let _data = data.into_inner();
    
    let mut _np = 0;

    if _data.data.types == "NON SME" {
      _np = _data.data.net_profit* 20/ 100;
    }
    else if _data.data.types == "SME" {
      _np = _data.data.income- _data.data.expenses;
      if _np >= 0 && _np <= 300000 {
        _np = 0;
      }
      else if _np > 300000 && _np <= 3000000{
        _np = _np* 15/ 100;
      }
      else if _np > 3000000 {
        _np = _np* 20/ 100;
      }
    }

    let _ra = "2015-08-05T08:40:51.620Z";

    let res_cit = ResCit {
      tax_value: _np,
      response_at: _ra.to_string(),
    };
     
    let response_body = json!(res_cit);

    HttpResponse::Created().json(response_body)
}