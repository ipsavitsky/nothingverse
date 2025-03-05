CREATE TABLE posts (
    id INTEGER PRIMARY KEY,
    generation_id INTEGER NOT NULL,
    timestamp TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (generation_id) REFERENCES generations (id)
);

CREATE TABLE generations (
    id INTEGER PRIMARY KEY,
    content TEXT NOT NULL
)
