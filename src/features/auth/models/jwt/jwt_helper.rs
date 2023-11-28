use super::TokenClaims;
use crate::cores::BaseError;
use jwt_simple::{
    algorithms::{HS256Key, MACLike},
    claims::Claims,
    reexports::coarsetime::Duration,
};
use tracing::error;

pub struct JwtHelper;

impl JwtHelper {
    #[tracing::instrument(name = "Generate JWT Token" skip(jwt_secret_key))]
    pub fn generate_jwt(
        user_id: &str,
        email: &str,
        jwt_secret_key: &[u8],
    ) -> Result<String, BaseError> {
        let token_claims: TokenClaims = TokenClaims {
            user_id: user_id.to_owned(),
            email: email.to_owned(),
        };

        let key = HS256Key::from_bytes(jwt_secret_key);
        let claims = Claims::with_custom_claims(token_claims, Duration::from_hours(2));

        let token = key.authenticate(claims).map_err(|err| {
            error!("{:?}", err);
            BaseError::InternalServerError(format!("Unable to generate JWT Token: {}", err))
        })?;

        Ok(token)
    }

    #[tracing::instrument(name = "Verify JWT Token")]
    pub fn verify_jwt_token(token: &str) -> Result<TokenClaims, BaseError> {
        let jwt_secret: String = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set!");

        let key = HS256Key::from_bytes(jwt_secret.as_bytes());
        let claims = key
            .verify_token::<TokenClaims>(token, None)
            .map_err(|err| {
                error!("{:?}", err);
                BaseError::InvalidAccessToken(format!("Invalid or No Token Provided: {}", err))
            })?;

        Ok(claims.custom)
    }
}
