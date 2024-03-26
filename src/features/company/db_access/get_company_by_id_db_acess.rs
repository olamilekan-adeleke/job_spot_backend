use sqlx::PgPool;
use uuid::Uuid;

use crate::{feature::company::models::company::Company, BaseError};

pub async fn get_company_by_id_db_acess(
    pool: &PgPool,
    company_id: &Uuid,
) -> Result<Company, BaseError> {
    let data = sqlx::query_as!(
        Company,
        r#" SELECT * FROM companies WHERE id = $1 "#,
        company_id,
    )
    .fetch_one(pool)
    .await?;

    Ok(data)
}
