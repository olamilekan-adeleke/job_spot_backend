use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TokenClaims {
    pub user_id: String,
    pub email: String,
}
