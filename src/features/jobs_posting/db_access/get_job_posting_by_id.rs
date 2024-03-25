use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    feature::{
        company::{
            db_access::get_company_by_id_db_acess::get_company_by_id_db_acess,
            models::company::Company,
        },
        jobs_posting::model::job_posting::JobPosting,
    },
    BaseError,
};

pub async fn get_job_posting_by_id(pool: &PgPool, id: &str) -> Result<JobPosting, BaseError> {
    let job_posting_id = Uuid::parse_str(id).map_err(|e| {
        tracing::error!(%e, "Invaild Job Posting ID: {}", id);
        BaseError::InvalidBody("Invaild Job Posting ID".to_string())
    })?;

    let row = sqlx::query!(
        "SELECT * FROM job_postings WHERE job_id = $1",
        job_posting_id
    )
    .fetch_one(pool)
    .await?;

    let company = get_company(pool, &row.company_id).await.map_err(|err| {
        tracing::error!(%err, "Failed to get company");
        let msg = "Invalid Company ID, Please check and try again!".to_string();
        BaseError::InvalidBody(msg)
    })?;

    let job_posting = JobPosting {
        job_id: row.job_id,
        title: row.title.to_string(),
        description: row.description.to_string(),
        position: row.position.to_string(),
        job_worktype: row.job_worktype.to_string(),
        location: row.location.to_string(),
        company_id: row.company_id.to_string(),
        company,
        job_type: row.job_type.to_string(),
        salary: row.salary,
        currency: row.currency.to_string(),
        created_at: row.created_at,
        updated_at: row.updated_at,
    };

    Ok(job_posting)
}

async fn get_company(pool: &PgPool, id: &str) -> Result<Company, BaseError> {
    let company_id = Uuid::parse_str(id).map_err(|error| {
        tracing::error!(%error, "Invalid company id: {}", id);
        BaseError::InvalidBody("Invalid company id".to_string())
    })?;

    let company = get_company_by_id_db_acess(pool, &company_id).await?;
    Ok(company)
}
