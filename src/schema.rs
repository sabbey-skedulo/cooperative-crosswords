// @generated automatically by Diesel CLI.

diesel::table! {
    crossword (id, series) {
        id -> Varchar,
        series -> Varchar,
        date -> Int8,
        crossword_json -> Jsonb,
    }
}

diesel::table! {
    solution (crossword_for, team_for) {
        crossword_for -> Varchar,
        team_for -> Varchar,
        solution_json -> Jsonb,
    }
}

diesel::allow_tables_to_appear_in_same_query!(crossword, solution,);
