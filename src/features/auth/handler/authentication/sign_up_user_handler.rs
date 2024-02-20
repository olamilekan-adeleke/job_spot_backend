use actix_web::{web, HttpResponse};
use validator::Validate;

use crate::{
    cores::{map_response_with_token, AppState, BaseError},
    feature::{
        db_access::{check_phone_exist_db, check_username_exist_db, create_new_user_db},
        middleware::password_hasher::password_hasher::PasswordHelper,
        models::{create_user_data::CreateUser, JwtHelper},
    },
};

#[tracing::instrument(name = "Creating Account for new user" skip(app_state))]
pub async fn sign_up_handler(
    app_state: web::Data<AppState>,
    mut user_data: web::Json<CreateUser>,
) -> Result<HttpResponse, BaseError> {
    // Validate Form
    user_data
        .validate()
        .map_err(|err| BaseError::InvalidInput(err.to_string()))?;
    tracing::info!("User data validated successfully");

    check_username_exist_db(&app_state.db, &user_data.username).await?;
    tracing::info!("Username was found to be unique");

    check_phone_exist_db(&app_state.db, &user_data.phone_number).await?;
    tracing::info!("Phone number was found to be unique");

    let hashed_password = PasswordHelper::hash_password(&user_data.password.to_owned())?;
    user_data.password = hashed_password;

    let user = create_new_user_db(&app_state.db, &user_data).await?;
    tracing::info!("User Account was successfully added to DB");

    let access_token = JwtHelper::generate_jwt(
        user.user_id.to_string().as_str(),
        user.email.as_str(),
        app_state.config.jwt_secret.as_bytes(),
    )?;
    tracing::info!("Generated JWT token for user");

    Ok(map_response_with_token(
        &user,
        "Account Created Successfully",
        &access_token,
    ))
}
