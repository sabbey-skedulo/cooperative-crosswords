extern crate futures;
extern crate serde;

use crate::models::api_models::SolutionItemDto;
use crate::models::db_models::SolutionItem;
use crate::models::errors::AppError;
use crate::services::solution_db_actions::{get_solution, store_or_update_solution};
use crate::DbPool;
use actix_web::web;
use std::collections::HashMap;

pub async fn update_solution(
    pool: web::Data<DbPool>,
    solution_items_api: Vec<SolutionItemDto>,
    user_id: String,
    team_id: String,
    crossword_id: String,
) -> Result<Vec<SolutionItemDto>, AppError> {
    let current_solution_items = get_solution(pool.clone(), crossword_id.clone(), team_id.clone())
        .await?
        .unwrap_or(Vec::new());

    let mut position_to_item: HashMap<(i64, i64), SolutionItem> = current_solution_items
        .into_iter()
        .map(|item| ((item.x, item.y), item))
        .collect();

    let new_solution_items: Vec<SolutionItem> = solution_items_api
        .iter()
        .map(|solution_item| SolutionItem {
            x: solution_item.x,
            y: solution_item.y,
            value: solution_item.value.to_owned(),
            modified_by: user_id.clone(),
        })
        .filter(|solution_item| {
            let position = (solution_item.x, solution_item.y);
            let existing_item = position_to_item.get(&position);
            match existing_item {
                None => true,
                Some(item) => item.value != solution_item.value,
            }
        })
        .collect();

    for solution_item in new_solution_items.clone() {
        let position = (solution_item.x, solution_item.y);
        position_to_item.insert(position, solution_item);
    }

    store_or_update_solution(
        pool.clone(),
        crossword_id.clone(),
        team_id.clone(),
        position_to_item.into_values().collect(),
    )
    .await?;
    Ok(new_solution_items
        .clone()
        .iter()
        .map(|solution_item| SolutionItemDto {
            x: solution_item.x,
            y: solution_item.y,
            value: solution_item.clone().value,
        })
        .collect())
}

pub async fn retrieve_and_send_solution(
    pool: web::Data<DbPool>,
    team_id: String,
    crossword_id: String,
) -> Result<String, AppError> {
    let solution_items = get_solution(pool, crossword_id, team_id)
        .await?
        .unwrap_or(Vec::new());
    serde_json::to_string(&solution_items).map_err(|e| AppError::InternalServerError(e.to_string()))
}
