use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Vat {
  question: String,
  choices: Vec<VatChoice>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VatChoice {
  choice: String,
  product_name: Option<String>,
  product_price: Option<String>,
  service_name: Option<String>,
  service_price: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DATA {
  types: String,
  product_name: String,
  product_price: i32,
  service_name: String,
  service_price: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InputVat {
  user_id: i32,
  data: DATA,
}