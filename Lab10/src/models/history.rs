use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Pit {
  pub assessable_income: u32,
  pub forty_1: u32,
  pub expenses: u32,
  pub net_income: u32,
  pub tax_reduction: u32,
  pub deductible_assessable_income: u32,
  pub tax_rate: String,
  pub individual_income_tax: u32,
  pub ladder_personal_income_tax: u32,
  pub recommend_choosing: String,
  pub response_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Cit {
  pub tax_value: u32,
  pub response_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Vat {
  pub vat: String,
  pub net_income: f32,
  pub response_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
  pub user_id: u32,
  pub pit: Vec<Pit>,
  pub cit: Vec<Cit>,
  pub vat: Vec<Vat>,
}