use serde::{ Deserialize, Serialize};

// Get
#[derive( Debug, Serialize, Deserialize)]
pub struct Vat {
  question: String,
  choices: Vec<VatChoice>,
}

#[derive( Debug, Serialize, Deserialize)]
pub struct VatChoice {
  choice: String,
  product_name: Option<String>,
  product_price: Option<String>,
  service_name: Option<String>,
  service_price: Option<String>,
}

// Post
/// Request
#[derive( Debug, Serialize, Deserialize)]
pub struct DATA {
  pub types: String,
  pub product_name: String,
  pub product_price: i32,
  pub service_name: String,
  pub service_price: i32,
}

#[derive( Debug, Serialize, Deserialize)]
pub struct InputVat {
  pub user_id: i32,
  pub data: DATA,
}

/// Response
#[derive(Serialize, Deserialize)]
pub struct ResVat {
  pub vat: String,
  pub net_income: i32,
  pub price: i32,
  pub response_at: String,
}