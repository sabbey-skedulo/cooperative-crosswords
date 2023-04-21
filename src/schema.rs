// @generated automatically by Diesel CLI.

diesel::table! {
    crossword (id, series) {
        id -> Varchar,
        series -> Varchar,
        date -> Int8,
        crossword_json -> Jsonb,
    }
}
