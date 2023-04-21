extern crate futures;
extern crate serde;

use actix_web::web;
use futures::future;
use scraper::Html;

use crate::models::db_models::Crossword;
use crate::models::guardian::GuardianCrossword;
use crate::services::db_actions::{get_crossword_ids_for_series, store_crosswords};
use crate::DbPool;

pub async fn scrape_crossword(series: &str, id: String) -> Result<GuardianCrossword, String> {
    let url = format!("https://www.theguardian.com/crosswords/{}/{}", series, id);
    let document = get_document(url).await?;
    let selector = scraper::Selector::parse(".js-crossword")
        .map_err(|e| format!("Invalid selector: {}", e.to_string()))?;
    let element = document
        .select(&selector)
        .last()
        .map_or(Err("No element found".to_string()), Ok)?;
    let json = element
        .value()
        .attr("data-crossword-data")
        .map_or(Err("No attribute found".to_string()), Ok)?;
    serde_json::from_str(&json)
        .map_err(|e| format!("Error parsing crossword data: {}", e.to_string()))
}

async fn get_document(url: String) -> Result<Html, String> {
    let response = reqwest::get(url)
        .await
        .map_err(|e| format!("Error retrieving url response: {}", e.to_string()))?
        .text()
        .await
        .map_err(|_| "Error getting text from response".to_string())?;
    Ok(Html::parse_document(&response))
}

async fn get_recent_crossword_ids(series: &str) -> Result<Vec<String>, String> {
    let url = format!("https://www.theguardian.com/crosswords/series/{}", series);
    let series_url = format!("https://www.theguardian.com/crosswords/{}", series);
    let document = get_document(url).await?;
    let selector = scraper::Selector::parse(".fc-item__container>a")
        .map_err(|e| format!("Invalid selector: {}", e.to_string()))?;
    Ok(document
        .select(&selector)
        .map(|s| s.value().attr("href"))
        .flatten()
        .map(|s| s.to_string())
        .filter(|url| url.starts_with(series_url.as_str()))
        .map(|url| {
            url.as_str()
                .replace(series_url.as_str(), "")
                .replace("/", "")
                .to_string()
        })
        .collect())
}

pub async fn update_crosswords(pool: web::Data<DbPool>) -> Result<String, String> {
    let series = "cryptic";
    let new_ids: Vec<String> = get_recent_crossword_ids(series).await?;
    let existing_crosswords_ids: Vec<String> =
        get_crossword_ids_for_series(pool.clone(), series.to_string()).await?;

    let new_crosswords: Vec<Crossword> = future::try_join_all(
        new_ids
            .iter()
            .filter(|crossword_id| !existing_crosswords_ids.contains(crossword_id))
            .map(|crossword_id| scrape_crossword(series, crossword_id.to_string())),
    )
    .await?
    .iter()
    .map(|guardian_crossword| Crossword {
        id: guardian_crossword.number.to_string(),
        series: series.to_string(),
        crossword_json: serde_json::to_value(guardian_crossword).unwrap(),
        date: guardian_crossword.date,
    })
    .collect();
    let updated_crosswords = store_crosswords(pool.clone(), new_crosswords).await?;
    Ok(format!(
        "Successfully scraped {} new crosswords",
        updated_crosswords.to_string()
    ))
}
