use serde::Deserialize;
use validator::{Validate, ValidationError};

#[derive(Debug, Validate, Deserialize)]
pub struct CreateJob {
    pub title: String,
    pub description: String,
    pub position: String,
    #[validate(length(min = 1), custom = "validate_worktype")]
    pub job_worktype: String,
    pub location: String,
    pub company_id: String,
    #[validate(length(min = 1), custom = "validate_jobtype")]
    pub job_type: String,
    pub salary: u64,
    pub currency: String,
}

fn validate_worktype(worktype: &str) -> Result<(), ValidationError> {
    let acceptted_type: [&str; 3] = ["on_site", "hybrid", "remote"];
    let is_vaild = acceptted_type.contains(&worktype);

    if !is_vaild {
        let msg = "worktype must be one of on_site, hybrid, remote";
        return Err(ValidationError::new(msg));
    }

    Ok(())
}

fn validate_jobtype(jobtype: &str) -> Result<(), ValidationError> {
    let acceptted_type: [&str; 6] = [
        "full_time",
        "part_time",
        "contract",
        "intern",
        "volunteer",
        "other",
    ];
    let is_vaild = acceptted_type.contains(&jobtype);

    if !is_vaild {
        let msg = "job_type must be one of on_site, hybrid, remote";
        return Err(ValidationError::new(msg));
    }

    Ok(())
}
