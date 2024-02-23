use actix_web::web;

use crate::authenticated_middleware::Autheticated;

use super::handlers::{create_company::create_company_handler, get_companies::get_compaines};

pub fn company_route(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/company")
            .wrap(Autheticated)
            .route("/", web::post().to(create_company_handler))
            .route("/", web::get().to(get_compaines)),
    );
}
