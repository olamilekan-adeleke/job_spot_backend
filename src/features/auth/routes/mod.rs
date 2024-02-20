use actix_web::web;

use super::handler::authentication::{
    forgot_password_handler::forgot_password_handler, login_handler::login_handler,
    sign_up_user_handler::sign_up_handler,
    verify_reset_password_handlers::verify_reset_password_handler,
};

pub fn auth_routes(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/v1").service(
            web::scope("/auth")
                .route("/login", web::post().to(login_handler))
                .route("/sign-up", web::post().to(sign_up_handler))
                .route("/forgot-password", web::post().to(forgot_password_handler))
                .route(
                    "/reset-password",
                    web::post().to(verify_reset_password_handler),
                ),
        ),
    );
}
