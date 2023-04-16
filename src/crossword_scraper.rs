use super::models::GuardianCrossword;

extern crate serde;

pub async fn scrape_crossword(series: &str, id: String) -> Result<GuardianCrossword, String> {
    let url = format!("https://www.theguardian.com/crosswords/{}/{}", series, id);
    let response = reqwest::get(url)
        .await
        .map_err(|e| format!("Error retrieving url response: {}", e.to_string()))?
        .text()
        .await
        .map_err(|_| "Error getting text from response".to_string())?;
    let document = scraper::Html::parse_document(&response);
    let selector = scraper::Selector::parse(".js-crossword").unwrap();
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
