use actix_web::{web, HttpResponse};

use crate::{
    cores::AppState,
    feature::jobs_posting::db_access::job_stats::job_stats_db_access::job_posting_stats_db_access,
    map_response, BaseError,
};

pub async fn job_stats_handler(app_state: web::Data<AppState>) -> Result<HttpResponse, BaseError> {
    let stats = job_posting_stats_db_access(&app_state.db).await?;

    let msg = "Fetched Job Posting Stats Successfully".to_string();
    Ok(map_response(&stats, &msg))
}
