use actix_web::web;

use super::handler::authentication::{
    login_handler::login_handler, sign_up_user_handler::sign_up_handler,
};

pub fn auth_routes(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/v1").service(
            web::scope("/auth")
                .route("/login", web::post().to(login_handler))
                .route("/sign-up", web::post().to(sign_up_handler)),
        ),
    );
}
