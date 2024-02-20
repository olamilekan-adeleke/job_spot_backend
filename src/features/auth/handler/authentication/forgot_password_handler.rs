use actix_web::{web, HttpResponse};
use chrono::NaiveDateTime;

use crate::feature::db_access::get_user_record_db;
use crate::feature::db_access::save_forgot_password_code::save_forgot_pasword_code;
use crate::feature::helpers::code_generator::CodeGenerator;
use crate::feature::models::VerificationData;
use crate::map_response;
use crate::{cores::AppState, feature::models::ForgotPassword, BaseError};

#[tracing::instrument(name = "Forgot Password Handler..." skip(app_state))]
pub async fn forgot_password_handler(
    app_state: web::Data<AppState>,
    data: web::Json<ForgotPassword>,
) -> Result<HttpResponse, BaseError> {
    let email = &data.email;

    let code = CodeGenerator::generate_random_code(6).unwrap();
    tracing::info!("{:?}", format!("Generated code:: {:?}", code));

    let user_data = get_user_record_db(&app_state.db, &email).await?;

    let mins_as_seconds = 60 * 20;
    let verification_data = VerificationData {
        user_id: user_data.user_id,
        code,
        expiration_date: NaiveDateTime::from_timestamp_opt(mins_as_seconds, 0).unwrap(),
    };

    let code_data = save_forgot_pasword_code(&app_state.db, verification_data).await?;
    Ok(map_response(
        &code_data,
        format!(
            "Verification Code Successfully Sent to your email addreess {}",
            email
        )
        .as_str(),
    ))
}
