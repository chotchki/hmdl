CREATE TABLE IF NOT EXISTS known_domains (
    name text NOT NULL,
    last_seen DATETIME NOT NULL,
    last_client text NOT NULL,
    training_input text NULL,
    PRIMARY KEY (name)
);