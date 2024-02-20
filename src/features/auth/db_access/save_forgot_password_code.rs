use sqlx::PgPool;

use crate::{feature::models::{VerificationCode, VerificationData}, BaseError};

pub async fn save_forgot_pasword_code(
    pool: &PgPool,
    data: VerificationData,
) -> Result<VerificationCode, BaseError> {

    let already_exit = sqlx::query_as!( VerificationCode, "SELECT * from verifications_code where user_id = $1", data.user_id, )
        .fetch_optional(pool).await?;


    let code_data: VerificationCode;
    if already_exit.is_some() {
        code_data = sqlx::query_as!(
            VerificationCode,
            "UPDATE verifications_code SET code = $1 WHERE user_id = $2 returning *",
            data.code,
            data.user_id
        ).fetch_one(pool) .await?;
    }else {
         code_data = sqlx::query_as!(
            VerificationCode,
            "INSERT into verifications_code (user_id, code, expiration_date) values ($1, $2, $3) returning *",
            data.user_id, 
            data.code, 
            data.expiration_date,
        ).fetch_one(pool).await?;
    }

    Ok(code_data)
}
