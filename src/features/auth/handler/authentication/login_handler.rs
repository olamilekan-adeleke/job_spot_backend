use actix_web::{web, HttpResponse};

use crate::{
    cores::{map_response_with_token, AppState, BaseError},
    feature::{
        db_access::get_user_record_db,
        middleware::password_hasher::password_hasher::PasswordHelper,
        models::{user::login_data::LoginData, JwtHelper},
    },
};

#[tracing::instrument(name = "Performing Login" skip(app_state))]
pub async fn login_handler(
    app_state: web::Data<AppState>,
    login_data: web::Json<LoginData>,
) -> Result<HttpResponse, BaseError> {
    let user = get_user_record_db(&app_state.db, &login_data.email).await?;
    tracing::info!("User details gotten from DB..");

    PasswordHelper::compare_password(&login_data.password, user.hash_password.as_str())?;

    let token = JwtHelper::generate_jwt(
        user.user_id.to_string().as_str(),
        user.email.as_str(),
        app_state.config.jwt_secret.as_bytes(),
    )?;
    tracing::info!("Generated JWT token for loggedIn user");

    tracing::info!("Return User Details");
    Ok(map_response_with_token(&user, "Login Successfully", &token))
}
