use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    feature::{company::models::company::Company, jobs_posting::model::job_posting::JobPosting},
    BaseError,
};

pub async fn get_job_postings_db_access(
    pool: &PgPool,
    page_size: i64,
) -> Result<Vec<JobPosting>, BaseError> {
    let job_posting_rows = sqlx::query!(
        "SELECT * FROM job_postings ORDER BY job_id LIMIT $1",
        page_size
    )
    .fetch_all(pool)
    .await?;

    let company_id_list = job_posting_rows
        .iter()
        .map(|row| Uuid::parse_str(&row.company_id).unwrap())
        .collect::<Vec<Uuid>>();

    let company_rows = sqlx::query!(
        "SELECT * FROM companies WHERE id = ANY($1)",
        &company_id_list
    )
    .fetch_all(pool)
    .await?;

    let mut job_posting_list = Vec::new();
    for row in job_posting_rows {
        let company_record = company_rows
            .iter()
            .find(|company| company.id == Uuid::parse_str(&row.company_id).unwrap())
            .unwrap();

        let company = Company {
            id: company_record.id,
            company_name: company_record.company_name.to_string(),
            category: company_record.category.to_string(),
            location: company_record.location.to_string(),
            year_created: company_record.year_created.to_string(),
            social_link: company_record.social_link.to_string(),
            website_link: company_record.website_link.to_string(),
            about: company_record.about.to_string(),
            industry: company_record.industry.to_string(),
            employee_size: company_record.employee_size,
            heads_office_address: company_record.heads_office_address.to_string(),
            specialization: company_record.specialization.to_string(),
            company_other_images: company_record.company_other_images.clone(),
            company_image_url: company_record.company_image_url.to_string(),
        };

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
        job_posting_list.push(job_posting);
    }

    Ok(job_posting_list)
}
