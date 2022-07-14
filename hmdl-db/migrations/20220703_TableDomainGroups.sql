CREATE TABLE IF NOT EXISTS domain_groups (
    name text NOT NULL,
    model_status text NOT NULL DEFAULT 'NEW',
    PRIMARY KEY (name)
);