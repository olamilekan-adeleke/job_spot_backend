use actix_web::{web, HttpResponse};

use crate::{authenticated_middleware::Autheticated, map_to_not_found_body_response, BaseError};

use super::handlers::{
    get_job_locations_handler::get_job_locations,
    get_job_position_handler::get_job_posistion_handler,
};

pub fn job_posting_routes(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/jobs")
            .wrap(Autheticated)
            .route("/positions", web::get().to(get_job_posistion_handler))
            .route("/locations", web::get().to(get_job_locations))
            .route("/stats", web::get().to(response))
            .route("/", web::get().to(next_part)),
    );
}

async fn response() -> String {
    format!("Welcome {}!", "to Job Posting")
}

async fn next_part() -> Result<HttpResponse, BaseError> {
    Ok(map_to_not_found_body_response(
        "Under Development".to_string(),
    ))
}
