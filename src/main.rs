use crate::models::errors::{to_status_code, AppError};
use actix::{Actor, Addr};
use actix_cors::Cors;
use actix_web::web::{Data, Path, Payload};
use actix_web::{
    get, middleware, post, App, Error, HttpRequest, HttpResponse, HttpServer, Responder,
};
use actix_web_actors::ws::start;
use diesel::r2d2;
use diesel::PgConnection;
use std::io::ErrorKind;

use crate::services::crossword_db_actions::{
    get_crossword_for_series_and_id, get_crossword_metadata_for_series,
};
use crate::services::ws_server::MoveServer;
use crate::services::ws_session::WsSession;

mod models;
mod schema;
mod services;

type DbPool = r2d2::Pool<r2d2::ConnectionManager<PgConnection>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=trace");
    env_logger::init();
    dotenv::dotenv().ok();
    let pool = initialize_db_pool()?;
    let server = MoveServer::new(pool.clone()).start();
    HttpServer::new(move || {
        App::new()
            .wrap(Cors::default().allow_any_method().allow_any_origin())
            .wrap(middleware::Logger::default())
            .app_data(Data::new(pool.clone()))
            .app_data(Data::new(server.clone()))
            .service(get_crossword_data)
            .service(get_all_crossword_data)
            .service(update_crosswords)
            .service(start_connection)
    })
    .bind(std::env::var("HOST_PORT").unwrap_or("127.0.0.1:8080".to_string()))?
    .run()
    .await
}

#[post("/update-crosswords")]
async fn update_crosswords(pool: Data<DbPool>) -> impl Responder {
    let result = services::crossword_service::update_crosswords(pool).await;
    return match result {
        Ok(message) => HttpResponse::Ok().body(message),
        Err(error) => build_error_response(error),
    };
}

#[get("/crossword/{id}")]
async fn get_crossword_data(pool: Data<DbPool>, path: Path<(String,)>) -> impl Responder {
    let crossword_id = path.into_inner().0;
    let crossword_data =
        get_crossword_for_series_and_id(pool, crossword_id, "cryptic".to_string()).await;
    return match crossword_data {
        Ok(message) => serde_json::to_string(&message).map_or(
            HttpResponse::BadRequest().body("Couldn't parse crossword to a string"),
            |x| HttpResponse::Ok().body(x),
        ),
        Err(error) => build_error_response(error),
    };
}

#[get("/crosswords")]
async fn get_all_crossword_data(pool: Data<DbPool>) -> impl Responder {
    let crossword_data = get_crossword_metadata_for_series(pool, "cryptic".to_string()).await;
    return match crossword_data {
        Ok(message) => serde_json::to_string(&message).map_or(
            HttpResponse::BadRequest().body("Couldn't parse metadata to a string"),
            |x| HttpResponse::Ok().body(x),
        ),
        Err(error) => build_error_response(error),
    };
}

#[get("/move/{team_id}/{crossword_id}/{user_id}")]
pub async fn start_connection(
    req: HttpRequest,
    stream: Payload,
    path: Path<(String, String, String)>,
    srv: Data<Addr<MoveServer>>,
) -> Result<HttpResponse, Error> {
    let ws = WsSession::new(
        srv.get_ref().clone(),
        path.2.clone(),
        path.clone().0,
        path.1.clone(),
    );
    start(ws, &req, stream)
}

fn build_error_response(error: AppError) -> HttpResponse {
    HttpResponse::build(to_status_code(error.clone())).body(error.clone().to_string())
}

fn initialize_db_pool() -> std::io::Result<DbPool> {
    let conn_spec = std::env::var("DATABASE_URL").map_err(|e| {
        std::io::Error::new(
            ErrorKind::ConnectionAborted,
            format!("Cannot read env variable DATABASE_URL: {}", e.to_string()),
        )
    })?;
    let manager = r2d2::ConnectionManager::<PgConnection>::new(conn_spec);
    r2d2::Pool::builder().max_size(32).build(manager).map_err(|e| {
        std::io::Error::new(
            ErrorKind::ConnectionAborted,
            format!("Cannot connect to database: {}", e.to_string()),
        )
    })
}
