use chrono::DateTime;
use serde::Serialize;

use crate::feature::company::models::company::Company;

#[derive(Debug, Serialize)]
pub struct JobPosting {
    pub job_id: uuid::Uuid,
    pub title: String,
    pub description: String,
    pub position: String,
    pub job_worktype: String,
    pub location: String,
    pub company_id: String,
    pub company: Company,
    pub job_type: String,
    pub salary: i64,
    pub currency: String,
    pub created_at: Option<DateTime<chrono::Utc>>,
    pub updated_at: Option<DateTime<chrono::Utc>>,
}
