use sqlx::{self, PgPool};

use crate::{cores::BaseError, feature::models::user_details::UserDetails};

pub async fn get_user_record_db(pool: &PgPool, email: &String) -> Result<UserDetails, BaseError> {
    let user = sqlx::query_as!(UserDetails, "SELECT * from users where email = $1", email,)
        .fetch_optional(pool)
        .await;

    let user = user.map_err(|err| {
        let msg = format!("Error Getting Record: {}", err);
        tracing::error!(msg);
        BaseError::DBError("Unable to get user record".to_string())
    })?;

    if let Some(user) = user {
        Ok(user)
    } else {
        let msg = format!("User with email '{}' address was not found!", email);
        Err(BaseError::InvalidInput(msg))
    }
}
