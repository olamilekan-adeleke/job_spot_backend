use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct VerificationCode {
    pub id: Uuid,
    pub user_id: Uuid,
    pub code: String,
    pub expiration_date: NaiveDateTime,
}

#[derive(Debug)]
pub struct VerificationData {
    pub user_id: Uuid,
    pub code: String,
    pub expiration_date: NaiveDateTime,
}
