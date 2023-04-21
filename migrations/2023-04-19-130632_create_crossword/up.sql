CREATE TABLE crossword
(
    id             VARCHAR NOT NULL ,
    series         VARCHAR NOT NULL,
    date           BIGINT  NOT NULL,
    crossword_json jsonb   NOT NULL,
    PRIMARY KEY (id, series)
)
