use actix_web::web;
use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct LoginData {
    #[validate(email(message = "must Be Valid Email Address"))]
    pub email: String,
    pub password: String,
}

impl From<web::Json<LoginData>> for LoginData {
    fn from(value: web::Json<LoginData>) -> Self {
        LoginData {
            email: value.email.clone(),
            password: value.password.clone(),
        }
    }
}
