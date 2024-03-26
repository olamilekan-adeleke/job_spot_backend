use actix_web::{web, HttpResponse};
use serde::Deserialize;

use crate::{
    cores::AppState,
    feature::jobs_posting::db_access::get_job_postings::get_job_postings_db_access::get_job_postings_db_access,
    map_response, BaseError,
};

#[derive(Debug, Deserialize)]
pub struct GetJobPostingQuery {
    pub page_size: Option<i64>,
    pub last_job_id: Option<String>,
}

pub async fn get_job_postings_handler(
    app_state: web::Data<AppState>,
    query: web::Query<GetJobPostingQuery>,
) -> Result<HttpResponse, BaseError> {
    let limit = query.0.page_size.unwrap_or(10);

    let jobs = get_job_postings_db_access(&app_state.db, limit).await?;

    let msg = "Fetched all jobs sucessfully!".to_string();
    Ok(map_response(&jobs, &msg))
}
