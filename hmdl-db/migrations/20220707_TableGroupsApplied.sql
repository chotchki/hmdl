CREATE TABLE IF NOT EXISTS groups_applied (
    client_group_name TEXT NOT NULL,
    domain_group_name TEXT NOT NULL,
    PRIMARY KEY (client_group_name, domain_group_name),
    FOREIGN KEY(client_group_name) REFERENCES client_group(name) ON DELETE CASCADE ON UPDATE CASCADE,
    FOREIGN KEY(domain_group_name) REFERENCES domain_group(name) ON DELETE CASCADE ON UPDATE CASCADE
);