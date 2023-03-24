use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Cit {
  question: String,
  choices: Vec<BusinessChoice>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BusinessChoice {
  choice: String,
  net_profit: Option<String>,
  income: Option<String>,
  expenses: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DATA {
  pub types: String,
  pub net_profit: i32,
  pub income: i32,
  pub expenses: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InputCit {
  pub user_id: i32,
  pub data: DATA,
}