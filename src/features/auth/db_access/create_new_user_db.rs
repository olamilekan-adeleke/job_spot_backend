use sqlx::PgPool;

use crate::{
    cores::BaseError,
    feature::models::user::{create_user_data::CreateUser, user_details::UserDetails},
};

pub async  fn create_new_user_db(pool: &PgPool, user_data: &CreateUser) -> Result<UserDetails, BaseError> {
    let user = sqlx::query_as!(
        UserDetails,
        "INSERT into users (email, full_name, username, hash_password) values ($1, $2, $3, $4) returning *",
        user_data.email,
        user_data.full_name,
        user_data.username,
        user_data.password,  
    ).fetch_one(pool).await?;

     Ok(user)
}
