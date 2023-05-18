CREATE TABLE crossword
(
    id             VARCHAR NOT NULL PRIMARY KEY,
    series         VARCHAR NOT NULL,
    series_no      BIGINT     NOT NULL,
    date           BIGINT  NOT NULL,
    crossword_json jsonb   NOT NULL
)
