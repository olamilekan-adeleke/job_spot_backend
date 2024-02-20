use actix_web::web;
use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct ForgotPassword {
    #[validate(email(message = "Must be a vali Emaill address"))]
    pub email: String,
}

impl From<web::Json<ForgotPassword>> for ForgotPassword {
    fn from(value: web::Json<ForgotPassword>) -> Self {
        ForgotPassword {
            email: value.email.clone(),
        }
    }
}
