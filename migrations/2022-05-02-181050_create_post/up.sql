CREATE TABLE post (
    post_id SERIAL PRIMARY KEY,
    date_posted TIMESTAMP NOT NULL,
    title VARCHAR(40) NOT NULL,
    text VARCHAR(1024) NOT NULL
);
