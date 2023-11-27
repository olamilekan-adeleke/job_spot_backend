use actix_web::HttpResponse;
use serde_json::json;

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
