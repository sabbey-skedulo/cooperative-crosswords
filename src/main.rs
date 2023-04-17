use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

mod crossword_scraper;
mod models;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(get_crossword_data)
            .service(get_all_crossword_data)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[get("/crossword/{id}")]
async fn get_crossword_data(path: web::Path<(String,)>) -> impl Responder {
    let crossword_id = path.into_inner().0;
    let crossword_data = crossword_scraper::scrape_crossword("cryptic", crossword_id).await;
    return match crossword_data {
        Ok(message) => serde_json::to_string(&message).map_or(
            HttpResponse::BadRequest().body("Couldn't parse crossword to a string"),
            |x| HttpResponse::Ok().body(x),
        ),
        Err(error) => HttpResponse::NotFound().body(error.to_string()),
    };
}

#[get("/crosswords")]
async fn get_all_crossword_data() -> impl Responder {
    let crossword_data = crossword_scraper::scrape_recent_crosswords("cryptic").await;
    return match crossword_data {
        Ok(message) => serde_json::to_string(&message).map_or(
            HttpResponse::BadRequest().body("Couldn't parse crossword to a string"),
            |x| HttpResponse::Ok().body(x),
        ),
        Err(error) => HttpResponse::NotFound().body(error.to_string()),
    };
}
