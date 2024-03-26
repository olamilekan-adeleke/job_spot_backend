use sqlx::PgPool;

use crate::{
    feature::company::models::{company::Company, create_new_company::CreateCompany},
    BaseError,
};

pub async fn create_company_db(
    pool: &PgPool,
    company: CreateCompany,
) -> Result<Company, BaseError> {
    let company = sqlx::query_as!(
        Company,
        r#"INSERT INTO 
            companies (
            company_image_url, company_name, category, location, year_created,
            social_link, website_link, about, industry, employee_size,
            heads_office_address, specialization, company_other_images
            ) 
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
        RETURNING *"#,
        company.company_image_url,
        company.company_name,
        company.category,
        company.location,
        company.year_created,
        company.social_link,
        company.website_link,
        company.about,
        company.industry,
        company.employee_size,
        company.heads_office_address,
        company.specialization,
        &company.company_other_images
    )
    .fetch_one(pool)
    .await?;

    Ok(company)
}
