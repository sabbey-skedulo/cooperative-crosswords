#![allow(unused)]
#![allow(clippy::all)]

use crate::schema::crossword;
use chrono::NaiveDate;
use diesel::Queryable;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Queryable)]
#[serde(rename_all = "camelCase")]
pub struct CrosswordMetadata {
    pub id: String,
    pub series: String,
    pub date: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SolutionItemApi {
    pub x: i64,
    pub y: i64,
    pub value: String,
}

