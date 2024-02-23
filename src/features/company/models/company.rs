use serde::Serialize;
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, FromRow)]
pub struct Company {
    pub id: Uuid,
    pub company_image_url: String,
    pub company_name: String,
    pub category: String,
    pub location: String,
    pub year_created: String,
    pub social_link: String,
    pub website_link: String,
    pub about: String,
    pub industry: String,
    pub employee_size: i32,
    pub heads_office_address: String,
    pub specialization: String,
    pub company_other_images: Vec<String>,
}
