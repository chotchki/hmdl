CREATE TABLE IF NOT EXISTS clients (
    name text NOT NULL,
    ip text NOT NULL,
    mac text NOT NULL,
    PRIMARY KEY (name),
    UNIQUE(ip),
    UNIQUE(mac)
);