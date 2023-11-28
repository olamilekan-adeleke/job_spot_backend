use actix_web::HttpResponse;
use serde::Serialize;
use serde_json::json;

pub fn map_response_with_token<T: Serialize>(data: &T, msg: &str, token: &str) -> HttpResponse {
    HttpResponse::Ok().json(json!({
            "status": "success",
            "message": msg,
            "token": token,
            "data": data
    }))
}

pub fn map_to_bad_body_response(msg: String) -> HttpResponse {
    let binding = msg.replace("Json deserialize error:", "");
    let mut msg = binding.as_str();

    if msg.contains("at line") {
        let line_at_index = msg.find("at line").unwrap_or(msg.len());
        msg = &msg[..line_at_index];
    }

    HttpResponse::BadRequest().json(json!({
            "state": "error",
            "message": msg.trim(),
            "status": 401,
    }))
}

pub fn map_to_not_found_body_response(msg: String) -> HttpResponse {
    let binding = msg.replace("Json deserialize error:", "");
    let mut msg = binding.as_str();

    if msg.contains("at line") {
        let line_at_index = msg.find("at line").unwrap_or(msg.len());
        msg = &msg[..line_at_index];
    }

    HttpResponse::NotFound().json(json!({
            "state": "error",
            "message": msg.trim(),
            "status": 404,
    }))
}
