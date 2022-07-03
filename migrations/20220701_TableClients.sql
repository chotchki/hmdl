CREATE TABLE IF NOT EXISTS clients (
    name text NOT NULL,
    ipv4 text NOT NULL,
    mac text NOT NULL,
    PRIMARY KEY (name),
    UNIQUE(ipv4),
    UNIQUE(mac)
);