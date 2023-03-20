use actix_web::{get, post, Responder, HttpResponse};

use crate::models::pit::{Pit};


#[get("/tax/pit")]
async fn get_pit() -> impl Responder {
    let url = "https://private-f5d89-ittipolcha.apiary-mock.com/tax/pit";
    let response = reqwest::get(url).await.unwrap().json::<Vec<Pit>>().await.unwrap();
    HttpResponse::Ok().json(response)
}

#[post("/tax/pit")]
async fn post_get(data: web::Json<RequestData>) -> impl Responder {
  // สร้าง Request แบบ POST พร้อมข้อมูลจาก Request body
  let client = reqwest::Client::new();
  let response = client
      .post("https://private-f5d89-ittipolcha.apiary-mock.com/tax/pit")
      .json(&data)
      .send()
      .await;

  // ตรวจสอบว่า Request สำเร็จหรือไม่
  match response {
      Ok(res) => {
          // อ่านข้อมูลจาก Response body
          let response_text = res.text().await.unwrap();
          // สร้าง Response ให้กับ Client
          HttpResponse::Ok().body(response_text)
      }
      Err(_) => {
          // สร้าง Response แจ้ง Error ให้กับ Client
          HttpResponse::InternalServerError().body("Error calling API")
      }
  }
}