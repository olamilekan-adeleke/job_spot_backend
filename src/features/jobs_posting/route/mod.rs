use actix_web::web;

use crate::authenticated_middleware::Autheticated;

use super::handlers::{
    create_job_posting_handler::create_job_posting_handler,
    get_job_locations_handler::get_job_locations,
    get_job_position_handler::get_job_posistion_handler,
};

pub fn job_posting_routes(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/jobs")
            .wrap(Autheticated)
            .route("/positions", web::get().to(get_job_posistion_handler))
            .route("/locations", web::get().to(get_job_locations))
            // .route("/stats", web::get().to(response))
            .route("/", web::post().to(create_job_posting_handler)),
    );
}