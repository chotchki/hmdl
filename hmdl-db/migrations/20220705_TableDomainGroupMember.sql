CREATE TABLE IF NOT EXISTS domain_group_member (
    domain_name TEXT NOT NULL,
    group_name TEXT NOT NULL,
    manually_set BOOLEAN NOT NULL,
    calculated_weight FLOAT NULL,
    PRIMARY KEY (domain_name, group_name),
    FOREIGN KEY(domain_name) REFERENCES known_domains(name) ON DELETE CASCADE ON UPDATE CASCADE,
    FOREIGN KEY(group_name) REFERENCES domain_groups(name) ON DELETE CASCADE ON UPDATE CASCADE
);