use sqlx::PgPool;
use uuid::Uuid;
use validator::Validate;

use crate::{
    feature::{
        company::db_access::get_company_by_id_db_acess::get_company_by_id_db_acess,
        jobs_posting::model::create_job::CreateJob,
    },
    BaseError,
};

pub async fn create_job_posting_db_access(
    pool: &PgPool,
    data: &CreateJob,
) -> Result<String, BaseError> {
    data.validate().map_err(|e| {
        tracing::error!(%e, "Invalid CreateJob struct");
        BaseError::InvalidBody(e.to_string())
    })?;

    let company_id = Uuid::parse_str(&data.company_id).map_err(|e| {
        tracing::error!(%e, "Invaild Company ID: {}", data.company_id);
        BaseError::InvalidBody("Invaild Company ID".to_string())
    })?;

    get_company_by_id_db_acess(pool, &company_id)
        .await
        .map_err(|err| {
            tracing::error!(%err, "Failed to get company");
            let msg = "No company ID found, Please check and try again!".to_string();
            BaseError::InvalidBody(msg)
        })?;

    let query_result = sqlx::query!(
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

    Ok(query_result.job_id.to_string())
}
