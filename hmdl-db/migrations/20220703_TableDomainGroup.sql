CREATE TABLE IF NOT EXISTS domain_group (
    name text NOT NULL,
    model_status text NOT NULL DEFAULT 'NEW',
    PRIMARY KEY (name)
);