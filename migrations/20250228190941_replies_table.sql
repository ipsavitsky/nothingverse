CREATE TABLE replies (
    id INTEGER PRIMARY KEY,
    content TEXT NOT NULL,
    post_id INTEGER NOT NULL,
    timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY(post_id) REFERENCES posts(id)
)
