use std::io;

use actix_web::{
    error::{InternalError, JsonPayloadError},
    web::{self, Data},
    App, HttpRequest, HttpResponse, HttpServer,
};
use cores::*;
use dotenv::dotenv;
use sqlx::PgPool;
use tracing::{debug, error};

use crate::{
    cores::states::{AppState, EnvConfig},
    feature::{company::routes::company_route, routes::auth_routes},
};

#[path = "../cores/mod.rs"]
mod cores;
#[path = "../features/mod.rs"]
mod feature;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .init();

    dotenv().ok();
    debug!("Loaded .env file..");

    let env_config: EnvConfig = EnvConfig::default();
    let db_pool = PgPool::connect(&env_config.database_url).await.unwrap();
    let app_state: AppState = AppState {
        db: db_pool,
        config: env_config.clone(),
    };

    let server_port = app_state.clone().config.server_port;

    let app = move || {
        {
            App::new()
                .app_data(Data::new(app_state.clone()))
                .app_data(web::JsonConfig::default().error_handler(json_error_handler))
                .configure(config)
                .service(
                    web::scope("/v1")
                        .configure(auth_routes)
                        .configure(company_route),
                )
                .default_service(web::route().to(not_found))
        }
    };

    debug!("Starting Serve on port: {}", server_port);
    let server_url = format!("127.0.0.1:{}", server_port);
    HttpServer::new(app).bind(server_url)?.run().await
}

fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/health").route(web::get().to(HttpResponse::Ok)));
}

async fn not_found(req: HttpRequest) -> Result<HttpResponse, BaseError> {
    let path = req.path().to_string();
    let msg = format!("The Route '{}' was not found", path);
    Ok(map_to_not_found_body_response(msg))
}

pub fn json_error_handler(err: JsonPayloadError, _: &HttpRequest) -> actix_web::Error {
    let err_msg = format!("Json Body Error: {}", err);
    error!(err_msg);

    InternalError::from_response("", map_to_bad_body_response(err.to_string())).into()
}
