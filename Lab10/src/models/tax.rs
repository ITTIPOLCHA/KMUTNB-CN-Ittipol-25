use serde::{Deserialize, Serialize};

/// Get
#[derive(Debug, Serialize, Deserialize)]
pub struct TaxChoice {
    pub choice: String,
    pub description: String,
    pub link: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tax {
    pub question: String,
    pub choices: Vec<TaxChoice>,
}