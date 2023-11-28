use actix_web::web;
use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate, Clone)]
pub struct CreateUser {
    #[validate(email(message = "Must Be Valid Email Address "))]
    pub email: String,
    #[validate(length(min = 3, message = "Must be above 3 char "))]
    pub full_name: String,
    #[validate(length(min = 2, max = 30, message = "Must be between 8 to 30 char "))]
    pub username: String,
    #[validate(length(min = 8, max = 50, message = "Must be between 8 to 50 char "))]
    pub password: String,
}

impl From<web::Json<CreateUser>> for CreateUser {
    fn from(value: web::Json<CreateUser>) -> Self {
        CreateUser {
            email: value.email.clone(),
            full_name: value.full_name.clone(),
            username: value.username.clone(),
            password: value.password.clone(),
        }
    }
}
