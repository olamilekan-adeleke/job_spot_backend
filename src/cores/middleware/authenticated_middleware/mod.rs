use crate::feature::models::{JwtHelper, TokenClaims};
use crate::BaseError;
use actix_web::dev::ServiceRequest;
use actix_web::{
    dev::{forward_ready, Service, ServiceResponse, Transform},
    http::header,
    Error, HttpMessage,
};
use futures_util::future::LocalBoxFuture;
use std::{
    future::{ready, Ready},
    rc::Rc,
};

pub struct Autheticated;

impl<S: 'static, B> Transform<S, ServiceRequest> for Autheticated
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AutheticatedMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AutheticatedMiddleware {
            service: Rc::new(service),
        }))
    }
}

pub struct AutheticatedMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for AutheticatedMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let srv = self.service.clone();
        println!("You requested: {}", req.path());

        Box::pin(async move {
            let token_claims = validate_token(&req)?;
            req.extensions_mut().insert(token_claims);

            let res = srv.call(req).await?;
            Ok(res)
        })
    }
}

fn validate_token(req: &ServiceRequest) -> Result<TokenClaims, BaseError> {
    let bearer_token = req.headers().get(header::AUTHORIZATION);

    if let Some(bearer_token) = bearer_token {
        let token = bearer_token.to_str().unwrap_or_default();
        let token_value = token.split_whitespace().nth(1).unwrap_or_default();

        tracing::info!("The User Token is:: {}", token_value);

        let token_claims = JwtHelper::verify_jwt_token(token_value)?;
        tracing::info!("token_claims is:: {:?}", token_claims);

        Ok(token_claims)
    } else {
        let msg = &"Unauthorized Request, You are not logged in, please provide token";

        Err(BaseError::InvalidAccessToken(msg.to_string()))
    }
}
