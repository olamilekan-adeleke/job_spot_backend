use actix_web::web;
use serde::Deserialize;

// TODO: Add Validation
#[derive(Debug, Deserialize)]
pub struct CreateCompany {
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

impl From<web::Json<CreateCompany>> for CreateCompany {
    fn from(value: web::Json<CreateCompany>) -> Self {
        CreateCompany {
            company_image_url: value.company_image_url.clone(),
            company_name: value.company_name.clone(),
            category: value.category.clone(),
            location: value.location.clone(),
            year_created: value.year_created.clone(),
            social_link: value.social_link.clone(),
            website_link: value.website_link.clone(),
            about: value.about.clone(),
            industry: value.industry.clone(),
            employee_size: value.employee_size,
            heads_office_address: value.heads_office_address.clone(),
            specialization: value.specialization.clone(),
            company_other_images: value.company_other_images.clone(),
        }
    }
}

// let company = CreateCompany {
//     company_image_url: "https://example.com/image.jpg".to_string(),
//     company_name: "Example Company".to_string(),
//     category: "Technology".to_string(),
//     location: "New York".to_string(),
//     year_created: "2020".to_string(),
//     social_link: "https://twitter.com/example".to_string(),
//     website_link: "https://example.com".to_string(),
//     about: "We make great products.".to_string(),
//     industry: "Software".to_string(),
//     employee_size:  100,
//     heads_office_address: "123 Example St".to_string(),
//     specialization: "Web Development".to_string(),
//     company_other_images: vec!["https://example.com/image2.jpg".to_string()],
// };
