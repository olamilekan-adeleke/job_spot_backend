// use actix_web::{web, HttpResponse};

use actix_web::{web, HttpResponse};

use crate::{
    cores::AppState,
    feature::jobs_posting::{
        db_access::{
            create_job_posting_db_access::create_job_posting_db_access,
            get_job_postings::get_job_posting_by_id_db_access::*,
        },
        model::create_job::CreateJob,
    },
    map_response, BaseError,
};

pub async fn create_job_posting_handler(
    app_state: web::Data<AppState>,
    data: web::Json<CreateJob>,
) -> Result<HttpResponse, BaseError> {
    // Get user Data
    let job_data = data.0;

    let job_id = create_job_posting_db_access(&app_state.db, &job_data).await?;
    tracing::info!("Added Job Posting To DB..");

    let job_posting = get_job_posting_by_id_db_access(&app_state.db, &job_id).await?;
    tracing::info!("Fetched Job Posting .....");

    let msg = "Create Job Posting Success";
    Ok(map_response(&job_posting, msg))
}

// "99ae2f1050-e6-48e9-91da-a67b22211e51",
// "b48571a1-6106-4f9f-b88b-d9a32195646a",
// "d5d704cb-327d-49f1-bf6a-8f1f42351c06",

// "5f0d19ed-0b79-40f0-a5e4-f57b58f80d6e",
// "5660b3fa-b774-45c5-bb05-2c40071e3be1",
//  "a69ce7d0-8732-47d2-b008-65bed7b77e69",

//  "62bc4c6e-5055-4b5d-a22b-0c3cdada1bb7",
// "5a584c8e-8364-4a0c-8515-974198cc4cb7",
//
// "9fa3fca8-a1d0-43c5-b1d6-3f743c78e9fa",
//  "f3d08cde-ea44-4307-a7 e4d-2886565530be",
