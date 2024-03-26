use actix_web::web;

use crate::authenticated_middleware::Autheticated;

use super::handlers::{
    create_job::create_job_posting_handler::create_job_posting_handler,
    get_job_locations_handler::get_job_locations,
    get_job_position_handler::get_job_posistion_handler,
    get_jobs::{
        get_job_posting_by_id_handler::get_job_posting_by_id_handler,
        get_job_postings_handler::get_job_postings_handler,
    },
    job_stats::job_stats_handler::job_stats_handler,
};

pub fn job_posting_routes(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/jobs")
            .wrap(Autheticated)
            .route("/positions", web::get().to(get_job_posistion_handler))
            .route("/locations", web::get().to(get_job_locations))
            //
            //Job Stats
            //
            .route("/stats", web::get().to(job_stats_handler))
            //
            // Get Jobs
            //
            .route("/:id", web::get().to(get_job_posting_by_id_handler))
            .route("/", web::get().to(get_job_postings_handler))
            //
            // Create Job
            //
            .route("/", web::post().to(create_job_posting_handler)),
    );
}
