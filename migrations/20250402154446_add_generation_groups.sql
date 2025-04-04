CREATE TABLE generation_groups (
    id INTEGER PRIMARY KEY,
    used BOOLEAN NOT NULL CHECK (used IN (0, 1)) DEFAULT 0,
    timestamp TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- create a default group that all generations get assigned to
INSERT INTO generation_groups (used) VALUES (1);

ALTER TABLE generations ADD COLUMN generation_group_id INTEGER REFERENCES generation_groups (id);

UPDATE generations SET generation_group_id = 1;
