use crate::services::db_actions::{
    get_crossword_for_series_and_id, get_crossword_metadata_for_series,
};
use actix_web::web::Data;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use diesel::r2d2;
use diesel::PgConnection;

mod models;
mod schema;
mod services;

type DbPool = r2d2::Pool<r2d2::ConnectionManager<PgConnection>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let pool = initialize_db_pool();
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pool.clone()))
            .service(get_crossword_data)
            .service(get_all_crossword_data)
            .service(update_crosswords)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[post("/update-crosswords")]
async fn update_crosswords(pool: Data<DbPool>) -> impl Responder {
    let result = services::crossword_service::update_crosswords(pool).await;
    return match result {
        Ok(message) => HttpResponse::Ok().body(message),
        Err(error) => HttpResponse::BadRequest().body(error.to_string()),
    };
}

#[get("/crossword/{id}")]
async fn get_crossword_data(pool: Data<DbPool>, path: web::Path<(String,)>) -> impl Responder {
    let crossword_id = path.into_inner().0;
    let crossword_data =
        get_crossword_for_series_and_id(pool, crossword_id, "cryptic".to_string()).await;
    return match crossword_data {
        Ok(message) => serde_json::to_string(&message).map_or(
            HttpResponse::BadRequest().body("Couldn't parse crossword to a string"),
            |x| HttpResponse::Ok().body(x),
        ),
        Err(error) => HttpResponse::NotFound().body(error.to_string()),
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
        Err(error) => HttpResponse::BadRequest().body(error.to_string()),
    };
}

fn initialize_db_pool() -> DbPool {
    let conn_spec = std::env::var("DATABASE_URL").expect("DATABASE_URL should be set");
    let manager = r2d2::ConnectionManager::<PgConnection>::new(conn_spec);
    r2d2::Pool::builder()
        .build(manager)
        .expect("database URL should be valid path to SQLite DB file")
}
