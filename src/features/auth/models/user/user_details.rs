use serde::Serialize;

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct UserDetails {
    pub user_id: uuid::Uuid,
    pub full_name: String,
    pub email: String,
    pub username: String,
    pub hash_password: String,
    pub phone_number: Option<String>,
    pub bio: Option<String>,
    pub profile_url: Option<String>,
    pub user_location: Option<String>,
    pub date_of_birth: Option<String>,
    pub gender: Option<String>,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}
