use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
};

use crate::{cores::BaseError, feature::models::user::create_user_data::CreateUser};

pub struct PasswordHelper;

impl PasswordHelper {
    #[tracing::instrument(name = "Hashing User password" skip_all)]
    pub fn hash_password(mut user_data: CreateUser) -> Result<CreateUser, BaseError> {
        let password = user_data.password.as_bytes();
        let salt = SaltString::generate(&mut OsRng);

        let argon2 = Argon2::default();
        let password_hash = argon2
            .hash_password(password, &salt)
            .map_err(|err| BaseError::NahMeFuckUp(err.to_string()))?;

        user_data.password = password_hash.to_string();
        tracing::info!("user password hashed and updated to hash password");

        Ok(user_data)
    }

    pub fn compare_password(password: &str, password_hash: &str) -> Result<(), BaseError> {
        let parsed_hash = PasswordHash::new(password_hash)
            .map_err(|err| BaseError::NahMeFuckUp(err.to_string()))?;

        let result = Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok();

        if result {
            Ok(())
        } else {
            let msg = "Incorrect password / User password does not match";
            tracing::error!(msg);
            Err(BaseError::InvalidInput(msg.to_string()))
        }
    }
}
