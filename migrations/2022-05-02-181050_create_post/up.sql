CREATE TABLE post (
    post_id SERIAL UNIQUE PRIMARY KEY,
    date_posted TIMESTAMP NOT NULL,
    title TEXT NOT NULL,
    text TEXT NOT NULL
);
