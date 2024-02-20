use sqlx::PgPool;
use uuid::Uuid;

use crate::{feature::models::user_details::UserDetails, BaseError};

pub async fn update_password_db(
    pool: &PgPool,
    password: &String,
    user_id: &Uuid,
) -> Result<UserDetails, BaseError> {
    let user_details = sqlx::query_as!(
        UserDetails,
        "UPDATE users SET hash_password = $1 where user_id = $2 returning *",
        password,
        user_id,
    )
    .fetch_one(pool)
    .await?;

    Ok(user_details)
}
