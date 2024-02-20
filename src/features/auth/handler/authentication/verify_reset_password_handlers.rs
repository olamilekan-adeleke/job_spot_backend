use actix_web::{web, HttpResponse};

use crate::{
    cores::AppState,
    feature::{
        db_access::{
            get_user_record_db, update_password_db::update_password_db,
            verify_reset_password_code_db::verify_reset_password_code_db,
        },
        middleware::password_hasher::password_hasher::PasswordHelper,
        models::reset_password::ResetPassword,
    },
    map_response, BaseError,
};

#[tracing::instrument(name = "Reset Password Handler..." skip(app_state))]
pub async fn verify_reset_password_handler(
    app_state: web::Data<AppState>,
    data: web::Json<ResetPassword>,
) -> Result<HttpResponse, BaseError> {
    let code = &data.code;
    let email = &data.email;

    if data.password != data.confrim_password {
        let msg = format!("Password does not match, Please check the password and try again!");
        return Err(BaseError::InvalidBody(msg));
    }
    let password = &data.password;

    tracing::info!("Verifying code sent..");
    let user_data = get_user_record_db(&app_state.db, &email).await?;
    let data = verify_reset_password_code_db(&app_state.db, &user_data.user_id).await?;

    if data.code != code.to_owned() {
        let msg = format!("Incorrect code, please check and try again!");
        return Err(BaseError::InvalidBody(msg));
    }
    tracing::info!("Code match sucessfully..");

    tracing::info!("Updating password..");
    let hashed_password = PasswordHelper::hash_password(&password)?;
    let _ = update_password_db(&app_state.db, &hashed_password, &user_data.user_id).await?;
    tracing::info!("New password updated to db..");

    Ok(map_response(&{}, "Reset password Sucessfull"))
}
