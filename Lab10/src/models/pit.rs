use serde::{Deserialize, Serialize};

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
  pub assessable_income: Vec<AIChoice>,
  pub less_expenses: Vec<LEChoice>,
  pub deductible: Vec<DChoice>,
}
