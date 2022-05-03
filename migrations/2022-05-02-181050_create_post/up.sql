CREATE TABLE Post (
    post_id SERIAL UNIQUE PRIMARY KEY,
    date_posted TIMESTAMP NOT NULL DEFAULT now(),
    title TEXT NOT NULL,
    text TEXT NOT NULL
);
