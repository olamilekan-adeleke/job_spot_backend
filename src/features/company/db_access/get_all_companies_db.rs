use sqlx::PgPool;

use crate::{feature::company::models::company::Company, BaseError};

pub async fn get_all_companies(
    pool: &PgPool,
    page_size: i64,
    last_company_name: Option<String>,
) -> Result<Vec<Company>, BaseError> {
    let company_list: Vec<Company>;

    if let Some(last_company_name) = last_company_name {
        company_list = sqlx::query_as!(
            Company,
            "SELECT * FROM companies WHERE company_name > $1 ORDER BY company_name LIMIT $2",
            last_company_name,
            page_size,
        )
        .fetch_all(pool)
        .await?;
    } else {
        company_list = sqlx::query_as!(
            Company,
            "SELECT * FROM companies ORDER BY company_name LIMIT $1",
            page_size,
        )
        .fetch_all(pool)
        .await?;
    }

    Ok(company_list)
}
