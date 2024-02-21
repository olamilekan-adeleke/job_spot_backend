use actix_web::web;

use crate::map_to_not_found_body_response;

pub fn job_posting_routes(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/v1").service(
            web::scope("/jobs")
                // .route(
                //     "/stats",
                //     web::get().to(|_| map_to_not_found_body_response("N")),
                // )
                // .route("/", web::get().to({})),
        ),
    );
}
