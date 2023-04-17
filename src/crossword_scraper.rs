use super::models::GuardianCrossword;

extern crate futures;
extern crate serde;
use futures::future;
use scraper::Html;

pub async fn scrape_crossword(series: &str, id: String) -> Result<GuardianCrossword, String> {
    let url = format!("https://www.theguardian.com/crosswords/{}/{}", series, id);
    scrape_crossword_at(url).await
}

async fn scrape_crossword_at(url: String) -> Result<GuardianCrossword, String> {
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
    Ok(scraper::Html::parse_document(&response))
}

pub async fn scrape_recent_crosswords(series: &str) -> Result<Vec<GuardianCrossword>, String> {
    let url = format!("https://www.theguardian.com/crosswords/series/{}", series);
    let document = get_document(url).await?;
    let selector = scraper::Selector::parse(".fc-item__container>a")
        .map_err(|e| format!("Invalid selector: {}", e.to_string()))?;
    future::try_join_all(
        document
            .select(&selector)
            .map(|s| s.value().attr("href"))
            .flatten()
            .map(|s| s.to_string())
            .filter(|url| {
                url.starts_with(format!("https://www.theguardian.com/crosswords/{}", series).as_str())
            })
            .map(|url| scrape_crossword_at(url)),
    )
    .await
}
