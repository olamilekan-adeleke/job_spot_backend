use actix_web::HttpResponse;
use serde::Serialize;

use crate::{map_response, BaseError};

pub async fn get_job_posistion_handler() -> Result<HttpResponse, BaseError> {
    let job_positions = JobPositions {
        positions: JOB_POSITIONS.to_vec(),
    };

    let msg = "Job Position Fetched Successfully";
    Ok(map_response(&job_positions.positions, msg))
}

#[derive(Serialize)]
struct JobPositions {
    positions: Vec<&'static str>,
}

static JOB_POSITIONS: [&str; 40] = [
    "Software Developer",
    "Front-End Developer",
    "Back-End Developer",
    "Full-Stack Developer",
    "Mobile App Developer",
    "iOS Developer",
    "Android Developer",
    "Game Developer",
    "DevOps Engineer",
    "Quality Assurance Engineer",
    "Data Scientist",
    "Machine Learning Engineer",
    "AI Specialist",
    "Blockchain Developer",
    "Cybersecurity Engineer",
    "Network Administrator",
    "IT Support Specialist",
    "Systems Administrator",
    "Database Administrator",
    "IT Project Manager",
    "IT Consultant",
    "IT Security Analyst",
    "Network Security Engineer",
    "Cloud Engineer",
    "Digital Marketing Specialist",
    "SEO Specialist",
    "Social Media Manager",
    "Content Marketer",
    "Sales Representative",
    "Account Manager",
    "Marketing Manager",
    "Sales Analyst",
    "Product Manager",
    "Brand Manager",
    "HR Manager",
    "Recruitment Specialist",
    "Employee Relations Officer",
    "Training and Development Manager",
    "Compliance Officer",
    "HR Analyst",
];
