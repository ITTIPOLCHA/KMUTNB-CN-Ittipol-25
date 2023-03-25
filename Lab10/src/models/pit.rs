use serde::{Deserialize, Serialize};

// get
#[derive(Debug, Serialize, Deserialize)]
pub struct AIChoice {
  pub forty_1: String,
  pub forty_2: String,
  pub forty_3: String,
  pub forty_4: String,
  pub forty_5: String,
  pub forty_6: String,
  pub forty_7: String,
  pub forty_8: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LEChoice {
  pub forty_1: String,
  pub forty_2: String,
  pub forty_3: String,
  pub forty_4: String,
  pub forty_5: String,
  pub forty_6: String,
  pub forty_7: String,
  pub forty_8: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DChoice {
  pub personal: i32,
  pub child_per_person: i32,
  pub parent_per_person: i32,
  pub disabled: i32,
  pub pregnancy_childbirth: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Pit {
  pub assessable_income: AIChoice,
  pub less_expenses: LEChoice,
  pub deductible: DChoice,
}

// post
/// request
#[derive(Debug, Serialize, Deserialize)]
pub struct InputPit {
  pub user_id: i32,
  pub assessable_income: AI,
  pub deductible: DChoice,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AI {
  pub forty_1: i32,
  pub forty_2: i32,
  pub forty_3: i32,
  pub forty_4: i32,
  pub forty_5: i32,
  pub forty_6: i32,
  pub forty_7: i32,
  pub forty_8: i32,
}

/// response
#[derive(Serialize, Deserialize)]
pub struct ResPit {
  pub assessable_income: i32,
  pub forty_1: i32,
  pub expenses: i32,
  pub net_income: i32,
  pub tax_reduction: i32,
  pub deductible_assessable_income: i32,
  pub tax_rate: String,
  pub individual_income_tax: i32,
  pub ladder_personal_income_tax: i32,
  pub recommend_choosing: String,
  pub response_at: String,
}