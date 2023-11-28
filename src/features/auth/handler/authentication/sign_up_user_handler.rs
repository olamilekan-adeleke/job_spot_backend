use actix_web::{web, HttpResponse};
use validator::Validate;

use crate::{
    cores::{map_to_response_with_token, AppState, BaseError},
    feature::{
        db_access::check_username_exist_db,
        db_access::create_new_user_db,
        models::{create_user_data::CreateUser, password_helper::PasswordHelper, JwtHelper},
    },
};

#[tracing::instrument(name = "Creating Account for new user" skip(app_state))]
pub async fn sign_up_handler(
    app_state: web::Data<AppState>,
    user_data: web::Json<CreateUser>,
) -> Result<HttpResponse, BaseError> {
    // Validate Form
    user_data
        .validate()
        .map_err(|err| BaseError::InvalidInput(err.to_string()))?;
    tracing::info!("User data validated successfully");

    check_username_exist_db(&app_state.db, &user_data.username).await?;
    tracing::info!("Username was found to be unique");

    let user_data_with_hashed_password = PasswordHelper::hash_password(user_data.into())?;

    let user = create_new_user_db(&app_state.db, &user_data_with_hashed_password).await?;
    tracing::info!("User Account was successfully added to DB");

    let access_token = JwtHelper::generate_jwt(
        user.user_id.to_string().as_str(),
        user.email.as_str(),
        app_state.config.jwt_secret.as_bytes(),
    )?;
    tracing::info!("Generated JWT token for user");

    Ok(map_to_response_with_token(
        &user,
        "Account Created Successfully",
        &access_token,
    ))
}
