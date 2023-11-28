use sqlx::{self, PgPool};

use crate::{cores::BaseError, feature::models::user::user_details::UserDetails};

pub async fn check_username_exist_db(pool: &PgPool, username: &str) -> Result<(), BaseError> {
    let user = sqlx::query_as!(
        UserDetails,
        "SELECT * from users where username = $1",
        username,
    )
    .fetch_optional(pool)
    .await?;

    if let Some(_) = user {
        let msg = format!("Username '{}' already exist in our records", username);
        tracing::error!(msg);
        Err(BaseError::DBError(msg))
    } else {
        Ok(())
    }
}
