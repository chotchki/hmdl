CREATE TABLE IF NOT EXISTS groups (
    name text NOT NULL,
    model_status integer NOT NULL DEFAULT 0,
    PRIMARY KEY (name)
);