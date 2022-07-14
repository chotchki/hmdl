CREATE TABLE IF NOT EXISTS client_group_member (
    client_name TEXT NOT NULL,
    group_name TEXT NOT NULL,
    PRIMARY KEY (client_name, group_name),
    FOREIGN KEY(client_name) REFERENCES clients(name) ON DELETE CASCADE ON UPDATE CASCADE,
    FOREIGN KEY(group_name) REFERENCES client_groups(name) ON DELETE CASCADE ON UPDATE CASCADE
);