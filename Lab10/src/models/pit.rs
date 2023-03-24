use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AIChoice {
  forty_1: String,
  forty_2: String,
  forty_3: String,
  forty_4: String,
  forty_5: String,
  forty_6: String,
  forty_7: String,
  forty_8: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AI {
  forty_1: i32,
  forty_2: i32,
  forty_3: i32,
  forty_4: i32,
  forty_5: i32,
  forty_6: i32,
  forty_7: i32,
  forty_8: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LEChoice {
  forty_1: String,
  forty_2: String,
  forty_3: String,
  forty_4: String,
  forty_5: String,
  forty_6: String,
  forty_7: String,
  forty_8: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DChoice {
  personal: i32,
  child_per_person: i32,
  parent_per_person: i32,
  disabled: i32,
  pregnancy_childbirth: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Pit {
  assessable_income: Vec<AIChoice>,
  less_expenses: Vec<LEChoice>,
  deductible: Vec<DChoice>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InputPit {
  user_id: i32,
  assessable_income: AI,
  deductible: DChoice,
}