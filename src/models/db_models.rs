#![allow(unused)]
#![allow(clippy::all)]

use crate::schema::crossword;
use chrono::NaiveDate;
use diesel::{Insertable, Queryable};
#[derive(Queryable, Debug, Clone)]
#[diesel(table_name = crossword)]
pub struct Crossword {
    pub id: String,
    pub series: String,
    pub date: i64,
    pub crossword_json: serde_json::Value,
}

#[derive(Insertable)]
#[diesel(table_name = crossword)]
pub struct InsertableCrossword<'a> {
    pub id: &'a str,
    pub series: &'a str,
    pub date: i64,
    pub crossword_json: serde_json::Value,
}
