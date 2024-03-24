use chrono::DateTime;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct JobPosting {
    pub job_id: uuid::Uuid,
    pub title: String,
    pub description: String,
    pub position: String,
    pub job_worktype: String,
    pub location: String,
    pub company_id: String,
    pub job_type: String,
    pub salary: i64,
    pub currency: String,
    pub created_at: Option<DateTime<chrono::Utc>>,
    pub updated_at: Option<DateTime<chrono::Utc>>,
}
