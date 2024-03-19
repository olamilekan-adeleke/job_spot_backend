use actix_web::HttpResponse;
use serde::Serialize;

use crate::{map_response, BaseError};

pub async fn get_job_locations() -> Result<HttpResponse, BaseError> {
    let job_location = JobLocations {
        locations: LOCATIONS_AND_STATES.to_vec(),
    };

    let msg = "Job Location Fetched Successfully";
    Ok(map_response(&job_location.locations, msg))
}

#[derive(Debug, Serialize)]
struct JobLocations {
    pub locations: Vec<&'static str>,
}

static LOCATIONS_AND_STATES: [&str; 48] = [
    "New York, New York",
    "Los Angeles, California",
    "Chicago, Illinois",
    "Houston, Texas",
    "Phoenix, Arizona",
    "Philadelphia, Pennsylvania",
    "San Antonio, Texas",
    "San Diego, California",
    "Dallas, Texas",
    "San Jose, California",
    "Lagos, Lagos",
    "Abuja, Federal Capital Territory",
    "Ibadan, Oyo",
    "Kano, Kano",
    "Port Harcourt, Rivers",
    "Jos, Plateau",
    "Kaduna, Kaduna",
    "Lagos, Lagos",
    "Abuja, Federal Capital Territory",
    "Ibadan, Oyo",
    "Kano, Kano",
    "Port Harcourt, Rivers",
    "Jos, Plateau",
    "Kaduna, Kaduna",
    "London, England",
    "Paris, France",
    "Berlin, Germany",
    "Tokyo, Japan",
    "Sydney, Australia",
    "Moscow, Russia",
    "Cairo, Egypt",
    "Jakarta, Indonesia",
    "Beijing, China",
    "Buenos Aires, Argentina",
    "Rio de Janeiro, Brazil",
    "Toronto, Canada",
    "Mexico City, Mexico",
    "Dubai, United Arab Emirates",
    "Tel Aviv, Israel",
    "Cape Town, South Africa",
    "Nairobi, Kenya",
    "Lagos, Nigeria",
    "Abuja, Nigeria",
    "Ibadan, Nigeria",
    "Port Harcourt, Nigeria",
    "Jos, Nigeria",
    "Kano, Nigeria",
    "Kaduna, Nigeria",
];
