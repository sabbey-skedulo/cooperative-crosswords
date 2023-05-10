CREATE TABLE solution
(
    crossword_for  VARCHAR NOT NULL,
    team_for       VARCHAR NOT NULL,
    solution_json jsonb   NOT NULL,
    PRIMARY KEY (crossword_for, team_for)
)
