use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct UserDetails {
    pub user_id: uuid::Uuid,
    pub full_name: String,
    pub email: String,
    pub username: String,
    pub hash_password: String,
    pub phone_number: String,
    pub bio: String,
    pub profile_url: String,
    pub user_location: String,
    pub date_of_birth: String,
    pub gender: String,
}
