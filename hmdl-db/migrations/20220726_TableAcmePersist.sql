CREATE TABLE IF NOT EXISTS acme_persist (
    application_domain text NOT NULL,
    cloudflare_api_token text NOT NULL,
    acme_email text NOT NULL,
    lock_column boolean NOT NULL DEFAULT true,
    PRIMARY KEY (lock_column),
    CONSTRAINT lock_column_singleton CHECK (lock_column == true)
);