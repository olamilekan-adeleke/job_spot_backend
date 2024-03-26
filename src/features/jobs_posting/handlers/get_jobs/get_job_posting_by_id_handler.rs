use actix_web::{web, HttpResponse};

use crate::{
    cores::AppState,
    feature::jobs_posting::db_access::get_job_postings::get_job_posting_by_id_db_access::*,
    map_response, BaseError,
};

pub async fn get_job_posting_by_id_handler(
    app_state: web::Data<AppState>,
    id: web::Path<String>,
) -> Result<HttpResponse, BaseError> {
    let job_posting = get_job_posting_by_id_db_access(&app_state.db, &id).await?;
    tracing::info!("Fetched Job Posting .....");

    let msg = "Fetched Job Posting Successfully".to_string();
    Ok(map_response(&job_posting, &msg))
}
