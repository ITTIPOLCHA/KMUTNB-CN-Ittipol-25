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
    pub published_at: String,
    pub choices: Vec<TaxChoice>,
}