use actix_web::{web, HttpResponse};

use crate::{
    cores::{map_to_bad_body_response, AppState, BaseError},
    feature::models::user::login_data::LoginData,
};

#[tracing::instrument(name = "Performing Login" skip(app_state))]
pub async fn login_handler(
    app_state: web::Data<AppState>,
    login_data: web::Json<LoginData>,
) -> Result<HttpResponse, BaseError> {
    // Get User Data form db
    tracing::info!("User details gotten from DB..");

    // compare user password with has password

    // Generate token claims for JWT

    tracing::info!("Return User Details");
    Ok(map_to_bad_body_response("No Implemented Yet".into()))
}
