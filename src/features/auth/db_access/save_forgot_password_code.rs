use sqlx::PgPool;

use crate::{feature::models::{VerificationCode, VerificationData}, BaseError};

pub async fn save_forgot_pasword_code(
    pool: &PgPool,
    data: VerificationData,
) -> Result<VerificationCode, BaseError> {
    let code_data = sqlx::query_as!(
        VerificationCode,
        "INSERT into verifications_code (user_id, code, expiration_date) values ($1, $2, $3) returning *",
        data.user_id, 
        data.code, 
        data.expiration_date,
    ).fetch_one(pool).await?;

    Ok(code_data)
}
