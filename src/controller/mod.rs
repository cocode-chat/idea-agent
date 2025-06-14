pub mod rag_controller;


use actix_web::{get, http::StatusCode, web, HttpResponse, HttpResponseBuilder, Responder};

// 快速返回结果
pub fn build_rpc_response<T: serde::Serialize>(code: StatusCode, msg: Option<String>, payload: Option<T>) -> impl Responder {
    if code == StatusCode::OK {
        let json_payload = match payload {
            Some(value) => serde_json::to_value(value).unwrap_or_else(|_| serde_json::json!({"error": "Failed to serialize payload"})),
            None => serde_json::json!({}),
        };
        HttpResponse::Ok().json(json_payload)
    } else {
        HttpResponseBuilder::new(code).json(serde_json::json!({"err_msg": msg}))
    }
}

pub fn cors() -> actix_cors::Cors {
    actix_cors::Cors::default()
        .allowed_origin("https://ideabase.io")
        .allowed_methods(vec!["*"])
        .allowed_headers(vec!["content-type"])
        .supports_credentials()
        .max_age(3600)
}

pub fn register_routes(cfg: &mut web::ServiceConfig) {
    // health
    cfg.service(health);
    // module scope
    cfg.service(web::scope("/api/v1")
        .service(rag_controller::scope()))
    ;
}

#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().json("I'm OK!")
}

