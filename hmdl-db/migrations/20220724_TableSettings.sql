CREATE TABLE IF NOT EXISTS hmdl_settings (
    application_domain text NOT NULL,
    cloudflare_api_token text NOT NULL,
    acme_email text NULL,
    lock_column boolean NOT NULL DEFAULT true,
    PRIMARY KEY (lock_column),
    CONSTRAINT lock_column_singleton CHECK (lock_column == true)
);