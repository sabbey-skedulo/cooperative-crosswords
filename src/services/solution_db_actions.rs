#![allow(unused)]

use actix_web::web;
use diesel::row::NamedRow;
use diesel::OptionalExtension;
use diesel::{ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl};
use serde_json::Value;

use crate::models::api_models::CrosswordMetadata;
use crate::models::db_models::{Crossword, Solution, SolutionItem};
use crate::models::errors::AppError;
use crate::models::errors::AppError::InternalServerError;
use crate::schema::solution::dsl::solution;
use crate::schema::solution::{crossword_for, solution_json, team_for};
use crate::DbPool;

pub async fn get_solution(
    pool: web::Data<DbPool>,
    crossword_id: String,
    team_id: String,
) -> actix_web::Result<Option<Vec<SolutionItem>>, AppError> {
    // use web::block to offload blocking Diesel queries without blocking server thread
    let result: Option<Value> = web::block(move || {
        let mut conn = pool.get()?;
        solution
            .filter(crossword_for.eq(crossword_id))
            .filter(team_for.eq(team_id))
            .select(solution_json)
            .first(&mut conn)
            .optional()
            .map_err(|x| AppError::InternalServerError(x.to_string()))
    })
    .await??;
    result.map_or(Ok(None), |x| {
        serde_json::from_value(x).map_err(|e| AppError::InternalServerError(e.to_string()))
    })
}

pub async fn store_or_update_solution(
    pool: web::Data<DbPool>,
    crossword_id: String,
    team_id: String,
    solution_items: Vec<SolutionItem>,
) -> actix_web::Result<usize, AppError> {
    // use web::block to offload blocking Diesel queries without blocking server thread1
    web::block(move || {
        let mut conn = pool.get()?;
        let solution_json_to_insert = serde_json::to_value(solution_items)?;
        let solution_to_insert = Solution {
            team_for: team_id,
            crossword_for: crossword_id,
            solution_json: solution_json_to_insert.clone(),
        };
        diesel::insert_into(solution)
            .values(&solution_to_insert)
            .on_conflict((team_for, crossword_for))
            .do_update()
            .set((solution_json.eq(solution_json_to_insert.clone())))
            .execute(&mut conn)
            .map_err(|e| AppError::InternalServerError(e.to_string()))
    })
    .await?
}
