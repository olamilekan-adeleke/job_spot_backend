use sqlx::PgPool;
use validator::Validate;

use crate::{
    feature::jobs_posting::model::create_job::CreateJob,
    feature::jobs_posting::model::job_posting::JobPosting, BaseError,
};

pub async fn create_job_posting_db_access(
    pool: &PgPool,
    data: CreateJob,
) -> Result<JobPosting, BaseError> {
    // Validate the CreateJob struct
    data.validate()
        .map_err(|e| BaseError::InvalidBody(e.to_string()))?;

    let data = sqlx::query_as!(
        JobPosting,
        r#"
            INSERT INTO job_postings (title, description, position, job_worktype, location, company_id, job_type, salary, currency)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            RETURNING *
        "#,
        data.title,
        data.description,
        data.position,
        data.job_worktype,
        data.location,
        data.company_id,
        data.job_type,
        data.salary,
        data.currency
    )
    .fetch_one(pool)
    .await?;

    Ok(data)
}
