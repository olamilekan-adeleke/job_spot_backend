use sqlx::PgPool;

use crate::{feature::jobs_posting::model::job_stats::JobStats, BaseError};

pub async fn job_posting_stats_db_access(pool: &PgPool) -> Result<JobStats, BaseError> {
    let query_result = sqlx::query!(
        r#"
            SELECT 
                COUNT(CASE WHEN job_type = 'on_site' THEN 1 END) AS on_site_count,
                COUNT(CASE WHEN job_type = 'remote' THEN 1 END) AS remote_count,
                COUNT(CASE WHEN job_type = 'hybrid' THEN 1 END) AS hybrid_count
            FROM job_postings

        "#,
    )
    .fetch_one(pool)
    .await?;

    let stats = JobStats {
        on_site_count: query_result.on_site_count.unwrap_or(0),
        remote_count: query_result.remote_count.unwrap_or(0),
        hybrid_count: query_result.hybrid_count.unwrap_or(0),
    };

    Ok(stats)
}
