use sqlx::PgPool;
use uuid::Uuid;

use crate::{feature::models::VerificationCode, BaseError};

pub async fn verify_reset_password_code_db(
    pool: &PgPool,
    user_id: &Uuid,
) -> Result<VerificationCode, BaseError> {
    let verification_data = sqlx::query_as!(
        VerificationCode,
        "SELECT * from verifications_code where user_id = $1",
        user_id,
    )
    .fetch_one(pool)
    .await?;

    Ok(verification_data)
}
