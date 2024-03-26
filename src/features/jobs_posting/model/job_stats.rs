use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct JobStats {
    pub on_site_count: i64,
    pub remote_count: i64,
    pub hybrid_count: i64,
}

