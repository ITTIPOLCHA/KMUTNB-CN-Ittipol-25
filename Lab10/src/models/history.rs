use serde::{Deserialize, Serialize};

// get
#[derive(Debug, Serialize, Deserialize)]
pub struct History {
    pub user_id: i32,
    pub pit: Vec<PIT>,
    pub cit: Vec<CIT>,
    pub vat: Vec<VAT>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PIT {
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

#[derive(Debug, Serialize, Deserialize)]
pub struct CIT {
    pub tax_value: i32,
    pub response_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VAT {
    pub vat: String,
    pub net_income: f32,
    pub response_at: String,
}