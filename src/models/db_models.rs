#![allow(unused)]
#![allow(clippy::all)]

use crate::schema::crossword;
use chrono::NaiveDate;
use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};
#[derive(Queryable, Debug, Clone, Insertable)]
#[diesel(table_name = crossword)]
pub struct Crossword {
    pub id: String,
    pub series: String,
    pub series_no: i64,
    pub date: i64,
    pub crossword_json: serde_json::Value,
}

use crate::schema::solution;
#[derive(Queryable, Debug, Clone, Insertable)]
#[diesel(table_name = solution)]
pub struct Solution {
    pub crossword_for: String,
    pub team_for: String,
    pub solution_json: serde_json::Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SolutionItem {
    pub x: i64,
    pub y: i64,
    pub value: String,
    pub modified_by: String,
}
