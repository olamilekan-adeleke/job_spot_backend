use sqlx::{self, PgPool};

use crate::{cores::BaseError, feature::models::user::user_details::UserDetails};

pub async fn check_phone_exist_db(pool: &PgPool, phone: &str) -> Result<(), BaseError> {
    let user = sqlx::query_as!(
        UserDetails,
        "SELECT * from users where phone_number = $1",
        phone,
    )
    .fetch_optional(pool)
    .await?;

    if user.is_none() {
        return Ok(());
    }

    let msg = format!("phone '{}' already belong to another user", phone);
    tracing::error!(msg);

    Err(BaseError::InvalidBody(msg))
}

// pub fn check_phone_avaliable(pool: &PgPool, phone: &str)-> Result<(), BaseError> {
//     let userdetail = sqlx::query_as!(
//      UserDetails,
//     "SELECT * from users where phone_number = $1"
//     phone,
//     );
//
//     Ok(())
//
// }
