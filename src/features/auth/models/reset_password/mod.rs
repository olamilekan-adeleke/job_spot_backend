use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct ResetPassword {
    pub email: String,
    pub code: String,
    #[validate(length(min = 8, max = 50, message = "Must be between 8 to 50 char "))]
    pub password: String,
    #[validate(length(min = 8, max = 50, message = "Must be between 8 to 50 char "))]
    pub confrim_password: String,
}
